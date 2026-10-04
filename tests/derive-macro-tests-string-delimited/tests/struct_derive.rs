use instrumentrs_macros::{__instrument_parameter, Parameter};

__instrument_parameter!();

/// An enum that could be a Channel.
#[derive(Debug, Parameter)]
#[cmd("{}")]
enum Channel {
    #[param("A")]
    ChA,
    #[param("B")]
    ChB,
    #[param("C")]
    ChC,
    #[param("D")]
    ChD,
}

// An enum that could be Some kind of status.
#[derive(Debug, Parameter)]
#[cmd("S-{}")]
enum ChStatus {
    #[param("OFF")]
    Off,
    #[param("ON")]
    On,
    #[param("UDEF")]
    Undefined,
}

#[test]
fn struct_no_ordering_2() {
    #[derive(Debug, Parameter)]
    #[cmd("SET {},{}")]
    struct State {
        channel: Channel,
        status: ChStatus,
    }

    let expected = "SET A,S-ON";

    let s = State {
        channel: Channel::ChA,
        status: ChStatus::On,
    };
    let received = s.to_writable();
    assert_eq!(received, expected);
}

#[test]
fn struct_full_ordering_2() {
    #[derive(Debug, Parameter)]
    #[cmd("SET {1},{0}")]
    struct State {
        channel: Channel,
        status: ChStatus,
    }

    let expected = "SET S-ON,A";

    let s = State {
        channel: Channel::ChA,
        status: ChStatus::On,
    };
    let received = s.to_writable();
    assert_eq!(received, expected);
}

#[test]
fn struct_no_ordering_4() {
    #[derive(Debug, Parameter)]
    #[cmd("CH 1{},1s{} : 2{},2s{}")]
    struct Multichannel {
        channel_1: Channel,
        status_1: ChStatus,
        channel_2: Channel,
        status_2: ChStatus,
    }

    let expected = "CH 1A,1sS-UDEF : 2C,2sS-OFF";

    let s = Multichannel {
        channel_1: Channel::ChA,
        status_1: ChStatus::Undefined,
        channel_2: Channel::ChC,
        status_2: ChStatus::Off,
    };
    let received = s.to_writable();
    assert_eq!(received, expected);
}

#[test]
fn struct_full_ordering_4() {
    #[derive(Debug, Parameter)]
    #[cmd("CH 1{2},1s{3} : 2{0},2s{1}")]
    struct Multichannel {
        channel_1: Channel,
        status_1: ChStatus,
        channel_2: Channel,
        status_2: ChStatus,
    }

    let expected = "CH 1C,1sS-OFF : 2A,2sS-UDEF";

    let s = Multichannel {
        channel_1: Channel::ChA,
        status_1: ChStatus::Undefined,
        channel_2: Channel::ChC,
        status_2: ChStatus::Off,
    };
    let received = s.to_writable();
    assert_eq!(received, expected);
}
