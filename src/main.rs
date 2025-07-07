use ble_peripheral_rust::{gatt::{characteristic::Characteristic, peripheral_event::{PeripheralEvent, ReadRequestResponse, RequestResponse, WriteRequestResponse}, properties::{AttributePermission, CharacteristicProperty}, service::Service}, Peripheral, PeripheralImpl};
use jackalope::{CHARACTERISTIC_UUID, SERVICE_UUID};
use tokio::sync::mpsc::channel;


#[tokio::main]
async fn main() {
    let be_peripheral = true;
    if be_peripheral {
	peripheral().await;
    }
}

async fn peripheral() {
        let (tx, mut rx) = channel::<PeripheralEvent>(256);
    let mut peripheral = Peripheral::new(tx).await.unwrap();
    

    while !peripheral.is_powered().await.unwrap() {}
    println!("powered!");

    let service = Service {
	uuid: SERVICE_UUID,
	primary: true,
	characteristics: vec![
	    Characteristic {
		uuid: CHARACTERISTIC_UUID,
		properties: vec![
		    CharacteristicProperty::Read,
		    CharacteristicProperty::Write,
		    CharacteristicProperty::WriteWithoutResponse,
		    CharacteristicProperty::Notify,
		],
		permissions: vec![
		    AttributePermission::Readable,
		    AttributePermission::Writeable,
		],
		value: None,
		descriptors: vec![],
	    },
	],
    };

    peripheral.add_service(&service).await.unwrap();

    peripheral.start_advertising("jackalope test", &[SERVICE_UUID]).await.unwrap();
    tokio::spawn (async move {
	println!("hhh");
	while let Some(event) = rx.recv().await {
	    println!("event!");
	    match event {
		PeripheralEvent::CharacteristicSubscriptionUpdate { request, subscribed } => {
		    println!("CharacteristicSubscriptionUpdate: Subscribed {subscribed} {request:?}")
		},
		PeripheralEvent::ReadRequest { request, offset, responder } => {
		    println!("ReadRequest: {request:?} offset: {offset}");
		    responder.send(ReadRequestResponse {
			value: String::from("hii").into(),
			response: RequestResponse::Success,
		    }).unwrap();
		},
		PeripheralEvent::StateUpdate { is_powered } => {
		    println!("StateUpdate: {is_powered:?}")
		},
		PeripheralEvent::WriteRequest { request, value, offset, responder } => {
		    println!("WriteRequest: {request:?} value: {value:?} offset: {offset}");
		    responder.send(WriteRequestResponse {
			response: RequestResponse::Success,
		    }).unwrap();
		}
	    }
	}});
    loop{}
}
