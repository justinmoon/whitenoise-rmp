mod logging;
mod tests;

use crossbeam::channel::{unbounded, Receiver, Sender};
use std::sync::Arc;

/// State updates sent from backend Model to frontend RmpViewModel
#[derive(Debug, PartialEq, Clone, uniffi::Enum)]
pub enum ModelUpdate {
    CountChanged { count: i32 },
}

/// Requests for state changes or side effects sent from
/// frontend RmpViewModel to backend Model
#[derive(Debug, PartialEq, uniffi::Enum)]
pub enum Action {
    Increment,
    Decrement,
}

/// ViewModel synchronizes state from Model to RmpViewModel on frontend
/// which is generated from ViewModel
#[derive(Clone)]
struct ViewModel(pub Sender<ModelUpdate>);

/// Source of truth for application state
/// RmpModel is generated from Model and callable from frontend
#[derive(Debug)]
pub struct Model {
    pub count: i32,
    pub data_dir: String,
    update_receiver: Arc<Receiver<ModelUpdate>>,
}

impl rust_multiplatform::traits::RmpAppModel for Model {
    type ActionType = Action;
    type UpdateType = ModelUpdate;

    fn create(data_dir: String) -> Self {
        // Create a channel, give sender to ViewModel and receiver to Model
        let (sender, receiver) = unbounded();
        ViewModel::init(sender);
        Model {
            count: 0,
            data_dir,
            update_receiver: Arc::new(receiver),
        }
    }

    fn action(&mut self, action: Self::ActionType) {
        match action {
            Action::Increment => self.count += 1,
            Action::Decrement => self.count -= 1,
        }
        ViewModel::model_update(ModelUpdate::CountChanged { count: self.count });
    }

    fn get_update_receiver(&self) -> Arc<Receiver<Self::UpdateType>> {
        self.update_receiver.clone()
    }
}

#[uniffi::export]
impl RmpModel {
    pub fn get_count(&self) -> i32 {
        self.get_or_set_global_model()
            .read()
            .expect("Failed to acquire read lock on model")
            .count
    }

    pub fn setup_logging(&self) {
        logging::init_logging();
    }
}

/// Generate RmpModel and RmpViewModel from these
rust_multiplatform::register_app!(Model, ViewModel, Action, ModelUpdate);
