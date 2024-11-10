
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "I2c",
            extends: None,
            description: Some(
                "Inter-integrated circuit",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "Control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "Control register 2",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oar1",
                    description: Some(
                        "Own address register 1",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Oar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oar2",
                    description: Some(
                        "Own address register 2",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Oar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "Data register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr1",
                    description: Some(
                        "Status register 1",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr2",
                    description: Some(
                        "Status register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "Clock control register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trise",
                    description: Some(
                        "TRISE register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trise",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltr",
                    description: Some(
                        "FLTR register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some(
                "Clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccr",
                    description: Some(
                        "Clock control register in Fast/Standard mode (Master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "duty",
                    description: Some(
                        "Fast mode duty cycle",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Duty",
                    ),
                },
                Field {
                    name: "f_s",
                    description: Some(
                        "I2C master mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "FS",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "Control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pe",
                    description: Some(
                        "Peripheral enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smbus",
                    description: Some(
                        "SMBus mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Smbus",
                    ),
                },
                Field {
                    name: "smbtype",
                    description: Some(
                        "SMBus type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Smbtype",
                    ),
                },
                Field {
                    name: "enarp",
                    description: Some(
                        "ARP enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enpec",
                    description: Some(
                        "PEC enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "engc",
                    description: Some(
                        "General call enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nostretch",
                    description: Some(
                        "Clock stretching disable (Slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "start",
                    description: Some(
                        "Start generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stop",
                    description: Some(
                        "Stop generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ack",
                    description: Some(
                        "Acknowledge enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pos",
                    description: Some(
                        "Acknowledge/PEC Position (for data reception)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pos",
                    ),
                },
                Field {
                    name: "pec",
                    description: Some(
                        "Packet error checking",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alert",
                    description: Some(
                        "SMBus alert",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swrst",
                    description: Some(
                        "Software reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "Control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "freq",
                    description: Some(
                        "Peripheral clock frequency",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iterren",
                    description: Some(
                        "Error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "itevten",
                    description: Some(
                        "Event interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "itbufen",
                    description: Some(
                        "Buffer interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaen",
                    description: Some(
                        "DMA requests enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "last",
                    description: Some(
                        "DMA last transfer",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "Data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dr",
                    description: Some(
                        "8-bit data register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fltr",
            extends: None,
            description: Some(
                "FLTR register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dnf",
                    description: Some(
                        "Digital noise filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Dnf",
                    ),
                },
                Field {
                    name: "anoff",
                    description: Some(
                        "Analog noise filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Oar1",
            extends: None,
            description: Some(
                "Own address register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "add",
                    description: Some(
                        "Interface address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addmode",
                    description: Some(
                        "Addressing mode (slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Addmode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Oar2",
            extends: None,
            description: Some(
                "Own address register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "endual",
                    description: Some(
                        "Dual addressing mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Endual",
                    ),
                },
                Field {
                    name: "add2",
                    description: Some(
                        "Interface address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr1",
            extends: None,
            description: Some(
                "Status register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Start bit (Master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addr",
                    description: Some(
                        "Address sent (master mode)/matched (slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btf",
                    description: Some(
                        "Byte transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "add10",
                    description: Some(
                        "10-bit header sent (Master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stopf",
                    description: Some(
                        "Stop detection (slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxne",
                    description: Some(
                        "Data register not empty (receivers)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txe",
                    description: Some(
                        "Data register empty (transmitters)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "berr",
                    description: Some(
                        "Bus error",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arlo",
                    description: Some(
                        "Arbitration lost (master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "af",
                    description: Some(
                        "Acknowledge failure",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr",
                    description: Some(
                        "Overrun/Underrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pecerr",
                    description: Some(
                        "PEC Error in reception",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timeout",
                    description: Some(
                        "Timeout or t_low detection flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alert",
                    description: Some(
                        "SMBus alert",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr2",
            extends: None,
            description: Some(
                "Status register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msl",
                    description: Some(
                        "Master/slave",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy",
                    description: Some(
                        "Bus busy",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tra",
                    description: Some(
                        "Transmitter/receiver",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gencall",
                    description: Some(
                        "General call address (Slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smbdefault",
                    description: Some(
                        "SMBus device default address (Slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smbhost",
                    description: Some(
                        "SMBus host header (Slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dualf",
                    description: Some(
                        "Dual flag (Slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pec",
                    description: Some(
                        "Packet error checking register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Trise",
            extends: None,
            description: Some(
                "TRISE register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trise",
                    description: Some(
                        "Maximum rise time in Fast/Standard mode (Master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Addmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BIT7",
                    description: Some(
                        "7-bit addressing mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT10",
                    description: Some(
                        "10-bit addressing mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dnf",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NOFILTER",
                    description: Some(
                        "Digital filter disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FILTER1",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 1 tI2CCLK",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FILTER2",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 2 tI2CCLK",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FILTER3",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 3 tI2CCLK",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FILTER4",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 4 tI2CCLK",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FILTER5",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 5 tI2CCLK",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "FILTER6",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 6 tI2CCLK",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "FILTER7",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 7 tI2CCLK",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "FILTER8",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 8 tI2CCLK",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "FILTER9",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 9 tI2CCLK",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "FILTER10",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 10 tI2CCLK",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "FILTER11",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 11 tI2CCLK",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "FILTER12",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 12 tI2CCLK",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "FILTER13",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 13 tI2CCLK",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "FILTER14",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 14 tI2CCLK",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "FILTER15",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 15 tI2CCLK",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Duty",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DUTY2_1",
                    description: Some(
                        "Duty cycle t_low/t_high = 2/1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DUTY16_9",
                    description: Some(
                        "Duty cycle t_low/t_high = 16/9",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Endual",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SINGLE",
                    description: Some(
                        "Single addressing mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DUAL",
                    description: Some(
                        "Dual addressing mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FS",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "Standard mode I2C",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FAST",
                    description: Some(
                        "Fast mode I2C",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pos",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CURRENT",
                    description: Some(
                        "ACK bit controls the (N)ACK of the current byte being received",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NEXT",
                    description: Some(
                        "ACK bit controls the (N)ACK of the next byte to be received",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Smbtype",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DEVICE",
                    description: Some(
                        "SMBus Device",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HOST",
                    description: Some(
                        "SMBus Host",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Smbus",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "I2C",
                    description: Some(
                        "I2C Mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SMBUS",
                    description: Some(
                        "SMBus",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                