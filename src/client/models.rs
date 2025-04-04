//! Client for Anki model (note type) operations

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{AnkiError, Result};
use crate::http::{HttpRequestSender, RequestSender};
use crate::models::{Field, Model, ModelId, NoteId};

use super::request::{self, FindModelsByIdParams, ModelFieldNamesParams, ModelTemplatesParams};

/// Client for model-related operations
pub struct ModelClient {
    sender: Arc<HttpRequestSender>,
}

impl ModelClient {
    /// Creates a new ModelClient with the given request sender
    pub(crate) fn new(sender: Arc<HttpRequestSender>) -> Self {
        Self { sender }
    }

    /// Gets all models (note types) from Anki
    ///
    /// # Returns
    ///
    /// A list of all models in the Anki collection
    pub fn get_all(&self) -> Result<Vec<Model>> {
        let result: HashMap<String, u64> = self.sender.send("modelNamesAndIds", None::<()>)?;

        // For each model, fetch its fields
        let mut models = Vec::with_capacity(result.len());
        for (name, id) in result {
            let fields = self.get_fields_for_name(&name)?;

            models.push(Model::new(
                id,
                name,
                fields
                    .into_iter()
                    .enumerate()
                    .map(|(ord, name)| Field::new(name, ord))
                    .collect(),
            )?);
        }

        Ok(models)
    }

    /// Gets a model by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model to get
    ///
    /// # Returns
    ///
    /// The model with the given name, if it exists
    pub fn get_by_name(&self, name: &str) -> Result<Option<Model>> {
        let models = self.get_all()?;
        Ok(models.into_iter().find(|m| m.name() == name))
    }

    /// Gets a model by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the model to get
    ///
    /// # Returns
    ///
    /// The model with the given ID, if it exists
    pub fn get_by_id(&self, id: ModelId) -> Result<Option<Model>> {
        let ids = [id.0];
        let params = FindModelsByIdParams { model_ids: &ids };

        let model_details: Vec<request::ModelDetails> =
            self.sender.send("findModelsById", Some(params))?;

        if model_details.is_empty() {
            return Ok(None);
        }

        let model_detail = &model_details[0];

        // Extract fields from the model details
        let fields = model_detail
            .flds
            .iter()
            .map(|f| Field::new(f.name.clone(), f.ord as usize))
            .collect::<Vec<_>>();

        Ok(Some(Model::new(
            model_detail.id,
            model_detail.name.clone(),
            fields,
        )?))
    }

    /// Gets the fields for a model
    ///
    /// # Arguments
    ///
    /// * `model` - The model to get fields for
    ///
    /// # Returns
    ///
    /// A list of fields for the model
    pub fn get_fields(&self, model: &Model) -> Result<Vec<Field>> {
        // Simply return the fields from the model
        Ok(model.fields().to_vec())
    }

    /// Gets the field names for a model by name
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model to get fields for
    ///
    /// # Returns
    ///
    /// A list of field names for the model
    pub fn get_fields_for_name(&self, model_name: &str) -> Result<Vec<String>> {
        let params = ModelFieldNamesParams { model_name };

        self.sender.send("modelFieldNames", Some(params))
    }

    /// Gets the field names for a model by ID
    ///
    /// # Arguments
    ///
    /// * `model_id` - The ID of the model to get fields for
    ///
    /// # Returns
    ///
    /// A list of field names for the model
    pub fn get_fields_for_id(&self, model_id: ModelId) -> Result<Vec<String>> {
        // First, get the model by ID
        let model = self.get_by_id(model_id)?.ok_or_else(|| {
            AnkiError::ValidationError(format!("Model with ID {} not found", model_id.0))
        })?;

        // Then get the fields
        self.get_fields_for_name(model.name())
    }

    /// Gets the template names for a model
    ///
    /// # Arguments
    ///
    /// * `model` - The model to get templates for
    ///
    /// # Returns
    ///
    /// A list of template names for the model
    pub fn get_template_names(&self, model: &Model) -> Result<Vec<String>> {
        let params = ModelTemplatesParams {
            model_name: model.name(),
        };

        self.sender.send("modelTemplates", Some(params))
    }

    /// Gets the CSS styling for a model
    ///
    /// # Arguments
    ///
    /// * `model` - The model to get styling for
    ///
    /// # Returns
    ///
    /// The CSS styling for the model
    pub fn get_styling(&self, model: &Model) -> Result<String> {
        let params = request::ModelStylingParams {
            model_name: model.name(),
        };

        self.sender.send("modelStyling", Some(params))
    }

    /// Creates a new model
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model to create
    /// * `fields` - The field names for the model
    /// * `css` - The CSS styling for the model
    /// * `templates` - The templates for the model (name, front, back)
    ///
    /// # Returns
    ///
    /// The ID of the created model
    pub fn create_model(
        &self,
        model_name: &str,
        fields: &[&str],
        css: &str,
        templates: &[(&str, &str, &str)],
    ) -> Result<ModelId> {
        if model_name.is_empty() {
            return Err(AnkiError::ValidationError(
                "Model name cannot be empty".to_string(),
            ));
        }

        if fields.is_empty() {
            return Err(AnkiError::ValidationError(
                "Model must have at least one field".to_string(),
            ));
        }

        if templates.is_empty() {
            return Err(AnkiError::ValidationError(
                "Model must have at least one template".to_string(),
            ));
        }

        // Convert templates to the expected format
        let api_templates = templates
            .iter()
            .map(|(name, front, back)| {
                (
                    name.to_string(),
                    request::CardTemplate {
                        front: front.to_string(),
                        back: back.to_string(),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        let params = request::CreateModelParams {
            model_name,
            in_order_fields: fields,
            css,
            card_templates: api_templates,
        };

        let id = self.sender.send::<_, u64>("createModel", Some(params))?;
        Ok(ModelId(id))
    }

    /// Updates the styling of a model
    ///
    /// # Arguments
    ///
    /// * `model` - The model to update
    /// * `css` - The new CSS styling
    pub fn update_styling(&self, model: &Model, css: &str) -> Result<()> {
        let params = request::UpdateModelStylingParams {
            model: model.name(),
            css,
        };

        self.sender
            .send::<_, ()>("updateModelStyling", Some(params))
    }

    /// Gets notes that use a specific model
    ///
    /// # Arguments
    ///
    /// * `model` - The model to find notes for
    ///
    /// # Returns
    ///
    /// A list of note IDs that use the model
    pub fn find_notes_using_model(&self, model: &Model) -> Result<Vec<NoteId>> {
        // We use the search query "note:ModelName"
        let query = format!("note:{}", model.name());
        let params = request::FindNotesParams { query };

        let ids = self.sender.send::<_, Vec<u64>>("findNotes", Some(params))?;
        Ok(ids.into_iter().map(NoteId).collect())
    }
}
