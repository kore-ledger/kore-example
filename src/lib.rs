use serde::{Deserialize, Serialize};
use kore_contract_sdk as sdk;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Data {
    pub temperature: f32,
    pub humidity: u32,
}

// Define the events of the contract.
#[derive(Serialize, Deserialize, Clone)]
enum Events {
    RegisterData { temperature: f32, humidity: u32 },
}

#[unsafe(no_mangle)]
pub unsafe fn main_function(state_ptr: i32, event_ptr: i32, is_owner: i32) -> u32 {
    sdk::execute_contract(state_ptr, event_ptr, is_owner, contract_logic)
}

#[unsafe(no_mangle)]
pub unsafe fn init_check_function(state_ptr: i32) -> u32 {
    sdk::check_init_data(state_ptr, init_logic)
}

fn init_logic(_state: &Data, contract_result: &mut sdk::ContractInitCheck) {
    contract_result.success = true;
}


fn contract_logic(
    context: &sdk::Context<Data, Events>,
    contract_result: &mut sdk::ContractResult<Data>,
) {
    let state = &mut contract_result.final_state;
    match context.event {
        Events::RegisterData {
            temperature,
            humidity,
        } => {
            if temperature < -20_f32 || temperature > 60_f32 || humidity > 100 {
                return;
            }
            state.humidity = humidity;
            state.temperature = temperature;
        }
    }
    contract_result.success = true;
}

#[test]
fn test_change_data_fail1() {
    let init_state = Data {
        humidity: 0,
        temperature: 0_f32,
    };

    let context = sdk::Context {
        initial_state: init_state.clone(),
        event: Events::RegisterData {
            humidity: 255,
            temperature: -50_f32,
        },
        is_owner: false,
    };

    let mut result = sdk::ContractResult::new(init_state);
    contract_logic(&context, &mut result);

    assert_eq!(result.final_state.humidity, 0);
    assert_eq!(result.final_state.temperature, 0_f32);

    assert!(!result.success);
}

#[test]
fn test_change_data_fail2() {
    let init_state = Data {
        humidity: 0,
        temperature: 0_f32,
    };

    let context = sdk::Context {
        initial_state: init_state.clone(),
        event: Events::RegisterData {
            humidity: 55,
            temperature: 200_f32,
        },
        is_owner: false,
    };

    let mut result = sdk::ContractResult::new(init_state);
    contract_logic(&context, &mut result);

    assert_eq!(result.final_state.humidity, 0);
    assert_eq!(result.final_state.temperature, 0_f32);

    assert!(!result.success);
}

#[test]
fn test_change_data_fail3() {
    let init_state = Data {
        humidity: 0,
        temperature: 0_f32,
    };

    let context = sdk::Context {
        initial_state: init_state.clone(),
        event: Events::RegisterData {
            humidity: 150,
            temperature: -2_f32,
        },
        is_owner: false,
    };

    let mut result = sdk::ContractResult::new(init_state);
    contract_logic(&context, &mut result);

    assert_eq!(result.final_state.humidity, 0);
    assert_eq!(result.final_state.temperature, 0_f32);

    assert!(!result.success);
}

#[test]
fn test_change_data_ok1() {
    let init_state = Data {
        humidity: 0,
        temperature: 0_f32,
    };

    let context = sdk::Context {
        initial_state: init_state.clone(),
        event: Events::RegisterData {
            humidity: 55,
            temperature: -2_f32,
        },
        is_owner: false,
    };

    let mut result = sdk::ContractResult::new(init_state);
    contract_logic(&context, &mut result);

    assert_eq!(result.final_state.humidity, 55);
    assert_eq!(result.final_state.temperature, -2_f32);

    assert!(result.success);
}

#[test]
fn test_change_data_ok2() {
    let init_state = Data {
        humidity: 0,
        temperature: 0_f32,
    };

    let context = sdk::Context {
        initial_state: init_state.clone(),
        event: Events::RegisterData {
            humidity: 100,
            temperature: -20_f32,
        },
        is_owner: false,
    };

    let mut result = sdk::ContractResult::new(init_state);
    contract_logic(&context, &mut result);

    assert_eq!(result.final_state.humidity, 100);
    assert_eq!(result.final_state.temperature, -20_f32);

    assert!(result.success);
}

#[test]
fn test_change_data_ok3() {
    let init_state = Data {
        humidity: 0,
        temperature: 0_f32,
    };

    let context = sdk::Context {
        initial_state: init_state.clone(),
        event: Events::RegisterData {
            humidity: 0,
            temperature: 60_f32,
        },
        is_owner: false,
    };

    let mut result = sdk::ContractResult::new(init_state);
    contract_logic(&context, &mut result);

    assert_eq!(result.final_state.humidity, 0);
    assert_eq!(result.final_state.temperature, 60_f32);

    assert!(result.success);
}
