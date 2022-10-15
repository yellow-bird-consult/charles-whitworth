use std::collections::{HashMap, VecDeque};
use std::mem;

use tokio::sync::mpsc;

use super::messages::{MessageType, StateActorMessage};


/// This actor is responsible for managing the state of all the chat logs for all patients.
/// 
/// # Fields
/// * patient_queue: the order in which the patient chats were inserted into the state
/// * patient_logs: the chat logs in relation to the patient
/// * receiver: the receiver that accepts INPUT and OUTPUT messages through a channel
/// * sender: the sender that sends messages through a channel
#[derive(Debug)]
pub struct StateActor {
    pub patient_queue: VecDeque<i32>,
    pub patient_logs: HashMap<i32, Vec<String>>,
    pub receiver: mpsc::Receiver<StateActorMessage>,
    pub sender: mpsc::Sender<StateActorMessage>,
}

impl StateActor {
    
    /// The constructor for the StateActor struct.
    /// 
    /// # Arguments
    /// * receiver: the receiver that accepts INPUT and OUTPUT messages through a channel
    /// * sender: the sender that sends messages through a channel
    /// 
    /// # Returns
    /// The constructed state actor
    pub fn new(receiver: mpsc::Receiver<StateActorMessage>, sender: mpsc::Sender<StateActorMessage>) -> StateActor {
        let patient_queue: VecDeque<i32> = VecDeque::new();
        let patient_logs: HashMap<i32, Vec<String>> = HashMap::new();
        return StateActor {patient_queue, patient_logs, receiver, sender}
    }

    /// Gets message data from the ```self.patient_logs```.
    /// 
    /// # Arguments
    /// * patient_id: the ID of the patient's chat logs to be extracted
    /// 
    /// # Returns
    /// * chat log data that belongs to the patient
    pub fn get_message_data(&mut self, patient_id: i32) -> Vec<String> {
        let reference = self.patient_logs.get_mut(&patient_id).unwrap();
        let data = mem::take(reference);
        self.patient_logs.remove(&patient_id);
        return data
    }

    /// Inserts a message into the ```self.patient_logs```.
    /// 
    /// # Arguments
    /// * patient_id: the ID of the patient's chat logs to be inserted
    /// * message_data: the message data that is going to be inserted into the chat log
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

    /// Processes an incoming message to either send chat log data out or insert new chat data. 
    /// 
    /// # Arguments
    /// message: the message to be processed
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
        println!("{:?}", self.patient_logs);
        println!("{:?}", self.patient_queue);
    }

    /// Runs the actor throughout the lifetime of the program accepting messages. 
    pub async fn run(mut self) {
        println!("state actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }
}
