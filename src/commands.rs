use std::fmt::{Debug, Display};

pub struct StaticCommand {
    pub(crate) command: &'static str,
    pub(crate) response: &'static str,
}

pub const STATIC_COMMAND_LUT: [StaticCommand; 9] = [
    StaticCommand{
        command: "ATZ\n",
        response: "Elm 3.1 whatever\nother stuff\n\n>"
    },
    StaticCommand{
        command: "ATE0\n",
        response: "ATE0\nother stuff\n\n>"
    },
    StaticCommand{
        command: "ATH1\n",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATSP5\n",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATST64\n",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATS0\n",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATM0\n",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATAT1\n",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATSH8210F0\n",
        response: "\n\n\r>"
    }
];

#[repr(u8)]
enum DynamicCommand {
    Heartbeat,
    GetRpm,
    GetVoltage,
    GetCoolantTemp
}

impl Display for DynamicCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DynamicCommand::Heartbeat => "210001".to_string(),
            DynamicCommand::GetRpm => "210C01".to_string(),
            DynamicCommand::GetVoltage => "ATRV".to_string(),
            DynamicCommand::GetCoolantTemp => "210501".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Debug for DynamicCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let str = match self {
            DynamicCommand::Heartbeat => "Heartbeat: 210001".to_string(),
            DynamicCommand::GetRpm => "GetRpm: 210C011".to_string(),
            DynamicCommand::GetVoltage => "GetVoltage: ATRV".to_string(),
            DynamicCommand::GetCoolantTemp => "GetCoolantTemp: 2105011".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub fn generate_2_byte_kwp_response(request: u8, val: u16) -> [u8; 8] {
    // 84 F0 10 61 0C 00 00 F1 - num_bytes = 2, val = 0
    
    let val_bytes: [u8; 2] = val.to_be_bytes();
    
    let mut temp_sum:u8 = 0;
    
    let mut response: [u8; 8] = [0x84, 0xF0, 0x10, 0x61, request, val_bytes[0], val_bytes[1], 0x00];
    response.iter().for_each(|f| {temp_sum = temp_sum.overflowing_add(*f).0 });
    response[7] = temp_sum;
    response
}

pub fn generate_1_byte_kwp_response(request: u8, val: u8) -> [u8; 7] {
    // 83 F0 10 61 05 7E 67 - num_bytes = 1, val = 0x7E

    let mut temp_sum:u8 = 0;

    let mut response: [u8; 7] = [0x84, 0xF0, 0x10, 0x61, request, val, 0x00];
    response.iter().for_each(|f| {temp_sum = temp_sum.overflowing_add(*f).0 });
    response[6] = temp_sum;
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_generation_2_byte() {
        let generated_result = generate_2_byte_kwp_response(0x0C, 00);
        let actual_response: [u8; 8] = [0x84, 0xF0, 0x10, 0x61, 0x0C, 0x00, 0x00, 0xF1];
        assert_eq!(generated_result, actual_response, "test_response_generation_2_byte test 1 failed");

        let generated_result = generate_2_byte_kwp_response(0x0C, 0x1623);
        let actual_response: [u8; 8] = [0x84, 0xF0, 0x10, 0x61, 0x0C, 0x16, 0x23, 0x2a];
        assert_eq!(generated_result, actual_response, "test_response_generation_2_byte test 2 failed");
    }
    #[test]
    fn test_response_generation_1_byte() {
        let generated_result = generate_1_byte_kwp_response(0x05, 0x7E);
        let actual_response: [u8; 7] = [0x84, 0xF0, 0x10, 0x61, 0x05, 0x7E, 0x68];
        assert_eq!(generated_result, actual_response, "test_response_generation_1_byte test 1 failed");

        let generated_result = generate_1_byte_kwp_response(0x05, 0x7D);
        let actual_response: [u8; 7] = [0x84, 0xF0, 0x10, 0x61, 0x05, 0x7D, 0x67];
        assert_eq!(generated_result, actual_response, "test_response_generation_1_byte test 2 failed");
    }
}

