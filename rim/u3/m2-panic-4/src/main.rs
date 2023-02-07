enum DoorState {
    Opened,
    Closed,
}

enum DoorAction {
    Open,
    Close,
}

fn take_action(current_state: DoorState, action: DoorAction) {
    match (current_state, action) {
        (DoorState::Opened, DoorAction::Close) => {
            // code to close the door goes here
        }
        (DoorState::Closed, DoorAction::Open) => {
            // code to open the door goes here
        }
        // If you get here, a programming mistake has been made
        _ => unreachable!(),
    }
}

fn main() {
    take_action(DoorState::Opened, DoorAction::Open);
}
