

#[derive(Debug)]
pub enum MessageType {
    INPUT,
    OUTPUT,
    EMPTY
}


/// This struct defines messages being sent to the state actor.
/// 
/// # Fields
/// * message_type: the type of message, if we are sending an input or output message
/// * patient_id: the ID of the patient in which we update the data for
/// * single_data: if input, this field will be populated to be inserted
/// * block_data: if output, the entire chat history of the patient to be sent to the librarian server
#[derive(Debug)]
pub struct StateActorMessage {
    pub message_type: MessageType,
    pub patient_id: Option<i32>,
    pub single_data: Option<String>,
    pub block_data: Option<Vec<String>>
}


impl StateActorMessage {

    pub async fn send_to_librarian(&self) {

    }

}