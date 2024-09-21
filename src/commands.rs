use std::fmt::{Debug, Display};

pub struct StaticCommand {
    pub(crate) command: &'static str,
    pub(crate) response: &'static str,
}

pub const STATIC_COMMAND_LUT: [StaticCommand; 13] = [
    StaticCommand{
        command: "ATZ",
        response: "Elm 3.1 whatever\nother stuff\n\n>"
    },
    StaticCommand{
        command: "ATE0",
        response: "ATE0\nother stuff\n\n>"
    },
    StaticCommand{
        command: "ATH1",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATSP5",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATST64",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATS0",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATM0",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATAT1",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "ATSH8210F0",
        response: "\n\n\r>"
    },
    StaticCommand{
        command: "210001",
        response: "83F01061057E00000067\n\r>"
    },
    StaticCommand{                      // not actually static
        command: "210501",
        response: "83F01061057E67\n\r>"
    },
    StaticCommand{                      // not actually static
        command: "210C01",
        response: "84F010610C0000F1\n\r>"
    },
    StaticCommand{                      // not actually static
        command: "ATRV",
        response: "12.6V\n\r>"
    },
];



struct DynamicCommand {
    id: DynamicCommands,
    command_str: &'static str,
    request: u8,
}

#[repr(u8)]
enum DynamicCommands {
    GetRpm,
    GetCoolantTemp
}

impl Display for DynamicCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DynamicCommands::GetRpm => "210C01".to_string(),
            DynamicCommands::GetCoolantTemp => "210501".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Debug for DynamicCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let str = match self {
            DynamicCommands::GetRpm => "GetRpm: 210C011".to_string(),
            DynamicCommands::GetCoolantTemp => "GetCoolantTemp: 2105011".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub fn generate_kwp_response(request: u8, val: &[u8]) -> Vec<u8> {
    // 84 F0 10 61 0C 00 00 F1 - num_bytes = 2, val = 0
    
    let mut temp_sum:u8 = 0;
    
    let mut response: Vec<u8> = vec!(0x84, 0xF0, 0x10, 0x61, request);
    
    response.extend_from_slice(val);
    
    response.iter().for_each(|&f| {temp_sum = temp_sum.overflowing_add(f).0 });
    response.push(temp_sum);
    response
}




//31 32 2E 34 56 20 3E

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_generation_2_byte() {
        let generated_result = generate_kwp_response(0x0C, &0x0000u16.to_be_bytes());
        let actual_response: Vec<u8> = vec!(0x84, 0xF0, 0x10, 0x61, 0x0C, 0x00, 0x00, 0xF1);
        assert_eq!(generated_result, actual_response, "test_response_generation_2_byte test 1 failed");

        let generated_result = generate_kwp_response(0x0C, &0x1623u16.to_be_bytes());
        let actual_response: Vec<u8> = vec!(0x84, 0xF0, 0x10, 0x61, 0x0C, 0x16, 0x23, 0x2a);
        assert_eq!(generated_result, actual_response, "test_response_generation_2_byte test 2 failed");
    }
    #[test]
    fn test_response_generation_1_byte() {
        let generated_result = generate_kwp_response(0x05, &0x7Eu8.to_be_bytes());
        let actual_response: Vec<u8> = vec!(0x84, 0xF0, 0x10, 0x61, 0x05, 0x7E, 0x68);
        assert_eq!(generated_result, actual_response, "test_response_generation_1_byte test 1 failed");

        let generated_result = generate_kwp_response(0x05, &0x7Du8.to_be_bytes());
        let actual_response: Vec<u8> = vec!(0x84, 0xF0, 0x10, 0x61, 0x05, 0x7D, 0x67);
        assert_eq!(generated_result, actual_response, "test_response_generation_1_byte test 2 failed");
    }
}

