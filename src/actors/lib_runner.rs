use std::collections::HashMap;
use super::messages::{MessageType, StateActorMessage};
use tokio::sync::{mpsc, oneshot, mpsc::Sender};
use std::{thread, time};


pub struct LibRunnerActor {
    pub interval: i32,
    pub receiver: mpsc::Receiver<StateActorMessage>,
    pub sender: mpsc::Sender<StateActorMessage>,
}

impl LibRunnerActor {

    pub fn new(receiver: mpsc::Receiver<StateActorMessage>, sender: mpsc::Sender<StateActorMessage>, interval: i32) -> LibRunnerActor {
        return LibRunnerActor { interval, receiver, sender }
    }        

    pub async fn run(mut self) {
        println!("lib runner actor is running");
        let seconds = time::Duration::from_secs(4);

        loop {
            thread::sleep(seconds);
            let message = StateActorMessage{
                message_type: MessageType::OUTPUT,
                patient_id: Some(3),
                single_data: None,
                block_data: None
            };
            match self.sender.send(message).await {
                Ok(_) => {
                    let message = self.receiver.recv().await.unwrap();
                    
                    let client = reqwest::Client::new();
                    let res = client.post("http://httpbin.org/post")
                                              .body("the exact body that is sent")
                                              .send()
                                              .await.unwrap();
                    println!("{:?}", res);
                },
                Err(_) => {
                    println!("lib runner is failed to send message");
                }
            };
        }
    }
}
