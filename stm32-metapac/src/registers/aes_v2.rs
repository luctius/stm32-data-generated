
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Aes",
            extends: None,
            description: Some(
                "Advanced encryption standard hardware accelerator",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "sr",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "dinr",
                    description: Some(
                        "Data input register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dinr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doutr",
                    description: Some(
                        "Data output register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doutr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "keyr",
                    description: Some(
                        "Key register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    12,
                                    32,
                                    36,
                                    40,
                                    44,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Keyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ivr",
                    description: Some(
                        "Initialization vector register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ivr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "suspr",
                    description: Some(
                        "Suspend register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Suspr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "AES enable",
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
                    name: "datatype",
                    description: Some(
                        "Data type selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Datatype",
                    ),
                },
                Field {
                    name: "mode",
                    description: Some(
                        "Operating mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "chmod10",
                    description: Some(
                        "Chaining mode bit1 bit0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccfc",
                    description: Some(
                        "Computation Complete Flag Clear",
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
                    name: "errc",
                    description: Some(
                        "Error clear",
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
                    name: "ccfie",
                    description: Some(
                        "CCF flag interrupt enable",
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
                    name: "dmainen",
                    description: Some(
                        "Enable DMA management of data input phase",
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
                    name: "dmaouten",
                    description: Some(
                        "Enable DMA management of data output phase",
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
                    name: "gcmph",
                    description: Some(
                        "GCM or CCM phase selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Gcmph",
                    ),
                },
                Field {
                    name: "chmod2",
                    description: Some(
                        "Chaining mode bit2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "keysize",
                    description: Some(
                        "Key size selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "npblb",
                    description: Some(
                        "Number of padding bytes in last block of payload",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dinr",
            extends: None,
            description: Some(
                "Data input register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "din",
                    description: Some(
                        "Input data word",
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
            name: "Doutr",
            extends: None,
            description: Some(
                "Data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dout",
                    description: Some(
                        "Output data word",
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
            name: "Ivr",
            extends: None,
            description: Some(
                "Initialization vector register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ivi",
                    description: Some(
                        "Initialization vector input",
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
            name: "Keyr",
            extends: None,
            description: Some(
                "Key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "Cryptographic key",
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
            name: "Sr",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccf",
                    description: Some(
                        "Computation complete flag",
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
                    name: "rderr",
                    description: Some(
                        "Read error flag",
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
                    name: "wrerr",
                    description: Some(
                        "Write error flag",
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
                    name: "busy",
                    description: Some(
                        "Busy flag",
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
            ],
        },
        FieldSet {
            name: "Suspr",
            extends: None,
            description: Some(
                "Suspend register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "susp",
                    description: Some(
                        "AES suspend",
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
            name: "Datatype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "Word",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HALFWORD",
                    description: Some(
                        "Half-word (16-bit)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BYTE",
                    description: Some(
                        "Byte (8-bit)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BIT",
                    description: Some(
                        "Bit",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Gcmph",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INITPHASE",
                    description: Some(
                        "Init phase",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HEADERPHASE",
                    description: Some(
                        "Header phase",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PAYLOADPHASE",
                    description: Some(
                        "Payload phase",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FINALPHASE",
                    description: Some(
                        "Final phase",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MODE1",
                    description: Some(
                        "Encryption",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MODE2",
                    description: Some(
                        "Key derivation (or key preparation for ECB/CBC decryption)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MODE3",
                    description: Some(
                        "Decryption",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MODE4",
                    description: Some(
                        "Key derivation then single decryption",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
                