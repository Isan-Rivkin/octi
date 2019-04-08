pub mod ws_controller;

#[cfg(test)]
mod test {
    use crate::ws_controller;
    #[test]
    fn test_create_workspace() {
        ws_controller::create_workspace("./")
    }
}