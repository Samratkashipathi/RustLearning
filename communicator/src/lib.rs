// General flow if a mod contains sub module we should create folder and rename the main file as mod.rs and copy remaining sub modules inside that folder
mod network;

pub mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ::client::connect();
        // super::client::connect(); -> Super tells to move one module above and check for client module
    }
}