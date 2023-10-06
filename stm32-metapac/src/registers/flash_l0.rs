
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Flash",
        extends: None,
        description: Some("Flash"),
        items: &[
            BlockItem {
                name: "acr",
                description: Some("Access control register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Acr"),
                }),
            },
            BlockItem {
                name: "pecr",
                description: Some("Program/erase control register"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pecr"),
                }),
            },
            BlockItem {
                name: "pdkeyr",
                description: Some("Power down key register"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Pdkeyr"),
                }),
            },
            BlockItem {
                name: "pekeyr",
                description: Some("Program/erase key register"),
                array: None,
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Pekeyr"),
                }),
            },
            BlockItem {
                name: "prgkeyr",
                description: Some("Program memory key register"),
                array: None,
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Prgkeyr"),
                }),
            },
            BlockItem {
                name: "optkeyr",
                description: Some("Option byte key register"),
                array: None,
                byte_offset: 20,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Optkeyr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("Status register"),
                array: None,
                byte_offset: 24,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "optr",
                description: Some("Option byte register"),
                array: None,
                byte_offset: 28,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Optr"),
                }),
            },
            BlockItem {
                name: "wrprot",
                description: Some("Write Protection Register 1"),
                array: None,
                byte_offset: 32,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Wrprot"),
                }),
            },
            BlockItem {
                name: "wrprot2",
                description: Some("Write Protection Register 2"),
                array: None,
                byte_offset: 128,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Wrprot"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Wrprot",
            extends: None,
            description: Some("Write Protection Register"),
            bit_size: 32,
            fields: &[Field {
                name: "wrprot",
                description: Some("Write Protection"),
                bit_offset: 0,
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 32, stride: 1 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Pekeyr",
            extends: None,
            description: Some("Program/erase key register"),
            bit_size: 32,
            fields: &[Field {
                name: "pekeyr",
                description: Some("FLASH_PEC and data EEPROM key"),
                bit_offset: 0,
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Pecr",
            extends: None,
            description: Some("Program/erase control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pelock",
                    description: Some("FLASH_PECR and data EEPROM lock"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prglock",
                    description: Some("Program memory lock"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optlock",
                    description: Some("Option bytes block lock"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prog",
                    description: Some("Program memory selection"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data",
                    description: Some("Data EEPROM selection"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fix",
                    description: Some("Fixed time data write for Byte, Half Word and Word programming"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erase",
                    description: Some("Page or Double Word erase mode"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fprg",
                    description: Some("Half Page/Double Word programming mode"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parallelbank",
                    description: Some("Parallel bank mode"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopie",
                    description: Some("End of programming interrupt enable"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errie",
                    description: Some("Error interrupt enable"),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "obl_launch",
                    description: Some("Launch the option byte loading"),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Optr",
            extends: None,
            description: Some("Option byte register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdprot",
                    description: Some("Read protection"),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wprmod",
                    description: Some("Selection of protection mode of WPR bits"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bor_lev",
                    description: Some("BOR_LEV"),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some("Access control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some("Latency"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prften",
                    description: Some("Prefetch enable"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sleep_pd",
                    description: Some("Flash mode during Sleep"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "run_pd",
                    description: Some("Flash mode during Run"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "disab_buf",
                    description: Some("Disable Buffer"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pre_read",
                    description: Some("Pre-read data address"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("Status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bsy",
                    description: Some("Write/erase operations in progress"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eop",
                    description: Some("End of operation"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "endhv",
                    description: Some("End of high voltage"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ready",
                    description: Some("Flash memory module ready after low power mode"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrperr",
                    description: Some("Write protected error"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgaerr",
                    description: Some("Programming alignment error"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sizerr",
                    description: Some("Size error"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optverr",
                    description: Some("Option validity error"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderr",
                    description: Some("RDERR"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "notzeroerr",
                    description: Some("NOTZEROERR"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fwwerr",
                    description: Some("FWWERR"),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pdkeyr",
            extends: None,
            description: Some("Power down key register"),
            bit_size: 32,
            fields: &[Field {
                name: "pdkeyr",
                description: Some("RUN_PD in FLASH_ACR key"),
                bit_offset: 0,
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Prgkeyr",
            extends: None,
            description: Some("Program memory key register"),
            bit_size: 32,
            fields: &[Field {
                name: "prgkeyr",
                description: Some("Program memory key"),
                bit_offset: 0,
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Optkeyr",
            extends: None,
            description: Some("Option byte key register"),
            bit_size: 32,
            fields: &[Field {
                name: "optkeyr",
                description: Some("Option byte key"),
                bit_offset: 0,
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
