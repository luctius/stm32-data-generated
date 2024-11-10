
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "Flash",
            ),
            items: &[
                BlockItem {
                    name: "acr",
                    description: Some(
                        "Flash access control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "keyr",
                    description: Some(
                        "Flash key register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "optkeyr",
                    description: Some(
                        "Flash option key register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Flash status register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Flash control register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ar",
                    description: Some(
                        "Flash address register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obr",
                    description: Some(
                        "Option byte register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Obr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrpr",
                    description: Some(
                        "Write protection register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrpr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some(
                "Flash access control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some(
                        "LATENCY",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Latency",
                    ),
                },
                Field {
                    name: "prftbe",
                    description: Some(
                        "Prefetch buffer enable",
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
                    name: "prftbs",
                    description: Some(
                        "Prefetch buffer status",
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
            ],
        },
        FieldSet {
            name: "Ar",
            extends: None,
            description: Some(
                "Flash address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "far",
                    description: Some(
                        "Flash address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Flash control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Programming",
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
                    name: "per",
                    description: Some(
                        "Page erase",
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
                    name: "mer",
                    description: Some(
                        "Mass erase",
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
                    name: "optpg",
                    description: Some(
                        "Option byte programming",
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
                    name: "opter",
                    description: Some(
                        "Option byte erase",
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
                    name: "strt",
                    description: Some(
                        "Start",
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
                    name: "lock",
                    description: Some(
                        "Lock",
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
                    name: "optwre",
                    description: Some(
                        "Option bytes write enable",
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
                    name: "errie",
                    description: Some(
                        "Error interrupt enable",
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
                    name: "eopie",
                    description: Some(
                        "End of operation interrupt enable",
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
                    name: "force_optload",
                    description: Some(
                        "Force option byte loading",
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
            ],
        },
        FieldSet {
            name: "Obr",
            extends: None,
            description: Some(
                "Option byte register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opterr",
                    description: Some(
                        "Option byte error",
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
                    name: "rdprt",
                    description: Some(
                        "Read protection level status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rdprt",
                    ),
                },
                Field {
                    name: "wdg_sw",
                    description: Some(
                        "WDG_SW",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "WdgSw",
                    ),
                },
                Field {
                    name: "n_rst_stop",
                    description: Some(
                        "nRST_STOP",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "NRstStop",
                    ),
                },
                Field {
                    name: "n_rst_stdby",
                    description: Some(
                        "nRST_STDBY",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "NRstStdby",
                    ),
                },
                Field {
                    name: "n_boot0",
                    description: Some(
                        "nBOOT0",
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
                    name: "n_boot1",
                    description: Some(
                        "BOOT1",
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
                    name: "vdda_monitor",
                    description: Some(
                        "VDDA power supply supervisor enabled",
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
                    name: "ram_parity_check",
                    description: Some(
                        "RAM_PARITY_CHECK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "RamParityCheck",
                    ),
                },
                Field {
                    name: "boot_sel",
                    description: Some(
                        "BOOT_SEL",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BootSel",
                    ),
                },
                Field {
                    name: "data0",
                    description: Some(
                        "Data0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data1",
                    description: Some(
                        "Data1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "Flash status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bsy",
                    description: Some(
                        "Busy",
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
                    name: "pgerr",
                    description: Some(
                        "Programming error",
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
                    name: "wrprt",
                    description: Some(
                        "Write protection error",
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
                    name: "eop",
                    description: Some(
                        "End of operation",
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
            ],
        },
        FieldSet {
            name: "Wrpr",
            extends: None,
            description: Some(
                "Write protection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp",
                    description: Some(
                        "Write protect",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "BootSel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NBOOT0",
                    description: Some(
                        "BOOT0 signal is defined by nBOOT0 option bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BOOT0",
                    description: Some(
                        "BOOT0 signal is defined by BOOT0 pin value (legacy mode)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Latency",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "WS0",
                    description: Some(
                        "0 wait states",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WS1",
                    description: Some(
                        "1 wait state",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "NRstStdby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset generated when entering Standby mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NORESET",
                    description: Some(
                        "No reset generated",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "NRstStop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset generated when entering Stop mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NORESET",
                    description: Some(
                        "No reset generated",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "RamParityCheck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "RAM parity check enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "RAM parity check disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rdprt",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LEVEL0",
                    description: Some(
                        "Level 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEVEL1",
                    description: Some(
                        "Level 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEVEL2",
                    description: Some(
                        "Level 2",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "WdgSw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HARDWARE",
                    description: Some(
                        "Hardware watchdog",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some(
                        "Software watchdog",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                