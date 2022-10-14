use std::collections::{HashMap, VecDeque};
use super::messages::{MessageType, StateActorMessage};
use tokio::sync::{mpsc, oneshot, mpsc::Sender};
use std::mem;


#[derive(Debug)]
pub struct StateActor {
    pub patient_queue: VecDeque<i32>,
    pub patient_logs: HashMap<i32, Vec<String>>,
    pub receiver: mpsc::Receiver<StateActorMessage>,
    pub sender: mpsc::Sender<StateActorMessage>,
}

impl StateActor {
    
    pub fn new(receiver: mpsc::Receiver<StateActorMessage>, sender: mpsc::Sender<StateActorMessage>) -> StateActor {
        let patient_queue: VecDeque<i32> = VecDeque::new();
        let patient_logs: HashMap<i32, Vec<String>> = HashMap::new();
        return StateActor {patient_queue, patient_logs, receiver, sender}
    }

    pub fn get_message_data(&mut self, patient_id: i32) -> Vec<String> {
        let reference = self.patient_logs.get_mut(&patient_id).unwrap();
        let data = mem::take(reference);
        self.patient_logs.remove(&patient_id);
        return data
    }

    pub fn insert_message(&mut self, patient_id: i32, message_data: String) {

        match self.patient_logs.get_mut(&patient_id) {
            Some(patient_log) => {
                patient_log.push(message_data);
            }, 
            None => {
                self.patient_queue.push_back(patient_id);
                let mut patient_log = Vec::new();
                patient_log.push(message_data);
                self.patient_logs.insert(patient_id, patient_log);
            }
        }

    }

    async fn handle_message(&mut self, message: StateActorMessage) {
        println!("state actor is receiving a message");

        match message.message_type {
            MessageType::INPUT => {
                self.insert_message(message.patient_id.unwrap(), message.single_data.unwrap());
            },
            MessageType::OUTPUT => {
                match self.patient_queue.pop_front() {
                    Some(patient_id) => {
                        let data = self.get_message_data(patient_id);
                        let message = StateActorMessage {
                            message_type: MessageType::OUTPUT,
                            patient_id: Some(patient_id),
                            single_data: None,
                            block_data: Some(data)
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    },
                    None => {
                        let message = StateActorMessage {
                            message_type: MessageType::EMPTY,
                            patient_id: None,
                            single_data: None,
                            block_data: None
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    }
                }
            },
            MessageType::EMPTY => {
                panic!("empty messages should not be sent to the state actor");
            }
        }
        println!("{:?}", self);
    }

    pub async fn run(mut self) {
        println!("state actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }
}
