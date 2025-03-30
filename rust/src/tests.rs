use crossbeam::channel::unbounded;

use crate::{Action, ModelUpdate, RmpModel, ViewModel};

#[test]
fn test_model_creation() {
    // Create an RmpModel instance
    let model = RmpModel::new("test_dir".to_string());

    // Verify it has the right data_dir
    assert_eq!(model.data_dir, "test_dir");
}

#[test]
fn test_action_handling() {
    // Create an RmpModel instance
    let model = RmpModel::new("test_dir".to_string());

    // Call the action method
    model.action(Action::Increment);

    // Get the global model
    let global_model = model.get_or_set_global_model().read().unwrap();

    // Verify the action was handled
    assert_eq!(global_model.count, 1);
}

#[test]
fn test_view_model() {
    // Create a channel for the view model
    let (sender, receiver) = unbounded();

    // Initialize the view model
    ViewModel::init(sender);

    // Send a model update
    ViewModel::model_update(ModelUpdate::CountChanged { count: 42 });

    // Verify the update was sent
    if let Ok(update) = receiver.try_recv() {
        match update {
            ModelUpdate::CountChanged { count } => assert_eq!(count, 42),
        }
    } else {
        panic!("No update received");
    }
}
