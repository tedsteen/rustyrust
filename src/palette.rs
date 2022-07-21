pub const NTSC_PAL: [u8; 64 * 8 * 4] = [
    0x52, 0x52, 0x52, 0xff, 0x01, 0x1a, 0x51, 0xff, 0x0f, 0x0f, 0x65, 0xff, 0x23, 0x06, 0x63, 0xff,
    0x36, 0x03, 0x4b, 0xff, 0x40, 0x04, 0x26, 0xff, 0x3f, 0x09, 0x04, 0xff, 0x32, 0x13, 0x00, 0xff,
    0x1f, 0x20, 0x00, 0xff, 0x0b, 0x2a, 0x00, 0xff, 0x00, 0x2f, 0x00, 0xff, 0x00, 0x2e, 0x0a, 0xff,
    0x00, 0x26, 0x2d, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xa0, 0xa0, 0xa0, 0xff, 0x1e, 0x4a, 0x9d, 0xff, 0x38, 0x37, 0xbc, 0xff, 0x58, 0x28, 0xb8, 0xff,
    0x75, 0x21, 0x94, 0xff, 0x84, 0x23, 0x5c, 0xff, 0x82, 0x2e, 0x24, 0xff, 0x6f, 0x3f, 0x00, 0xff,
    0x51, 0x52, 0x00, 0xff, 0x31, 0x63, 0x00, 0xff, 0x1a, 0x6b, 0x05, 0xff, 0x0e, 0x69, 0x2e, 0xff,
    0x10, 0x5c, 0x68, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xfe, 0xff, 0xff, 0xff, 0x69, 0x9e, 0xfc, 0xff, 0x89, 0x87, 0xff, 0xff, 0xae, 0x76, 0xff, 0xff,
    0xce, 0x6d, 0xf1, 0xff, 0xe0, 0x70, 0xb2, 0xff, 0xde, 0x7c, 0x70, 0xff, 0xc8, 0x91, 0x3e, 0xff,
    0xa6, 0xa7, 0x25, 0xff, 0x81, 0xba, 0x28, 0xff, 0x63, 0xc4, 0x46, 0xff, 0x54, 0xc1, 0x7d, 0xff,
    0x56, 0xb3, 0xc0, 0xff, 0x3c, 0x3c, 0x3c, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xfe, 0xff, 0xff, 0xff, 0xbe, 0xd6, 0xfd, 0xff, 0xcc, 0xcc, 0xff, 0xff, 0xdd, 0xc4, 0xff, 0xff,
    0xea, 0xc0, 0xf9, 0xff, 0xf2, 0xc1, 0xdf, 0xff, 0xf1, 0xc7, 0xc2, 0xff, 0xe8, 0xd0, 0xaa, 0xff,
    0xd9, 0xda, 0x9d, 0xff, 0xc9, 0xe2, 0x9e, 0xff, 0xbc, 0xe6, 0xae, 0xff, 0xb4, 0xe5, 0xc7, 0xff,
    0xb5, 0xdf, 0xe4, 0xff, 0xa9, 0xa9, 0xa9, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x4b, 0x34, 0x32, 0xff, 0x00, 0x07, 0x2e, 0xff, 0x0b, 0x01, 0x41, 0xff, 0x1d, 0x00, 0x42, 0xff,
    0x30, 0x00, 0x31, 0xff, 0x3b, 0x00, 0x16, 0xff, 0x3d, 0x03, 0x00, 0xff, 0x2f, 0x09, 0x00, 0xff,
    0x1c, 0x10, 0x00, 0xff, 0x09, 0x16, 0x00, 0xff, 0x00, 0x18, 0x00, 0xff, 0x00, 0x15, 0x00, 0xff,
    0x00, 0x0d, 0x11, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x94, 0x71, 0x6c, 0xff, 0x17, 0x27, 0x67, 0xff, 0x30, 0x1c, 0x84, 0xff, 0x4f, 0x13, 0x85, 0xff,
    0x6b, 0x11, 0x6b, 0xff, 0x7c, 0x15, 0x41, 0xff, 0x7e, 0x21, 0x16, 0xff, 0x6a, 0x2c, 0x00, 0xff,
    0x4c, 0x38, 0x00, 0xff, 0x2d, 0x42, 0x00, 0xff, 0x16, 0x45, 0x00, 0xff, 0x0a, 0x40, 0x10, 0xff,
    0x08, 0x32, 0x39, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xee, 0xbc, 0xb5, 0xff, 0x5d, 0x67, 0xb0, 0xff, 0x7c, 0x59, 0xcf, 0xff, 0xa0, 0x4f, 0xd1, 0xff,
    0xc0, 0x4b, 0xb4, 0xff, 0xd3, 0x51, 0x84, 0xff, 0xd6, 0x60, 0x51, 0xff, 0xbf, 0x6d, 0x25, 0xff,
    0x9d, 0x7b, 0x0f, 0xff, 0x79, 0x87, 0x0e, 0xff, 0x5c, 0x8a, 0x22, 0xff, 0x4b, 0x84, 0x49, 0xff,
    0x49, 0x75, 0x7b, 0xff, 0x36, 0x23, 0x21, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xee, 0xbc, 0xb5, 0xff, 0xaf, 0x98, 0xb3, 0xff, 0xbd, 0x91, 0xc0, 0xff, 0xcd, 0x8d, 0xc0, 0xff,
    0xdb, 0x8b, 0xb5, 0xff, 0xe3, 0x8e, 0xa1, 0xff, 0xe4, 0x94, 0x8a, 0xff, 0xda, 0x9a, 0x75, 0xff,
    0xcc, 0xa1, 0x69, 0xff, 0xbc, 0xa5, 0x69, 0xff, 0xaf, 0xa7, 0x74, 0xff, 0xa7, 0xa4, 0x86, 0xff,
    0xa6, 0x9e, 0x9d, 0xff, 0x9d, 0x78, 0x73, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x2e, 0x45, 0x27, 0xff, 0x00, 0x13, 0x2c, 0xff, 0x00, 0x07, 0x3a, 0xff, 0x0a, 0x00, 0x35, 0xff,
    0x15, 0x00, 0x20, 0xff, 0x1f, 0x00, 0x08, 0xff, 0x21, 0x03, 0x00, 0xff, 0x19, 0x0d, 0x00, 0xff,
    0x0c, 0x19, 0x00, 0xff, 0x01, 0x25, 0x00, 0xff, 0x00, 0x2c, 0x00, 0xff, 0x00, 0x28, 0x00, 0xff,
    0x00, 0x1f, 0x13, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x67, 0x8b, 0x5c, 0xff, 0x08, 0x3d, 0x64, 0xff, 0x18, 0x2a, 0x79, 0xff, 0x2d, 0x1a, 0x72, 0xff,
    0x40, 0x12, 0x51, 0xff, 0x50, 0x16, 0x29, 0xff, 0x52, 0x22, 0x04, 0xff, 0x46, 0x34, 0x00, 0xff,
    0x31, 0x48, 0x00, 0xff, 0x1c, 0x59, 0x00, 0xff, 0x0c, 0x64, 0x00, 0xff, 0x02, 0x5e, 0x13, 0xff,
    0x01, 0x50, 0x3d, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xae, 0xe1, 0x9d, 0xff, 0x3c, 0x89, 0xa6, 0xff, 0x52, 0x72, 0xbd, 0xff, 0x6c, 0x5f, 0xb6, 0xff,
    0x81, 0x54, 0x92, 0xff, 0x94, 0x5a, 0x64, 0xff, 0x96, 0x68, 0x34, 0xff, 0x89, 0x7e, 0x12, 0xff,
    0x71, 0x95, 0x04, 0xff, 0x56, 0xa9, 0x08, 0xff, 0x42, 0xb6, 0x21, 0xff, 0x33, 0xaf, 0x48, 0xff,
    0x31, 0x9f, 0x7a, 0xff, 0x1e, 0x31, 0x18, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xae, 0xe1, 0x9d, 0xff, 0x7d, 0xbb, 0xa1, 0xff, 0x86, 0xb1, 0xaa, 0xff, 0x92, 0xa9, 0xa7, 0xff,
    0x9b, 0xa4, 0x99, 0xff, 0xa3, 0xa7, 0x85, 0xff, 0xa4, 0xad, 0x70, 0xff, 0x9e, 0xb7, 0x5f, 0xff,
    0x94, 0xc1, 0x56, 0xff, 0x89, 0xca, 0x59, 0xff, 0x7f, 0xcf, 0x66, 0xff, 0x78, 0xcc, 0x79, 0xff,
    0x77, 0xc5, 0x8f, 0xff, 0x6e, 0x93, 0x62, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x2d, 0x2e, 0x1e, 0xff, 0x00, 0x04, 0x21, 0xff, 0x00, 0x00, 0x2f, 0xff, 0x0a, 0x00, 0x2d, 0xff,
    0x15, 0x00, 0x1d, 0xff, 0x1f, 0x00, 0x06, 0xff, 0x20, 0x00, 0x00, 0xff, 0x18, 0x05, 0x00, 0xff,
    0x0b, 0x0c, 0x00, 0xff, 0x00, 0x13, 0x00, 0xff, 0x00, 0x16, 0x00, 0xff, 0x00, 0x13, 0x00, 0xff,
    0x00, 0x0b, 0x0c, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x66, 0x66, 0x4d, 0xff, 0x07, 0x23, 0x52, 0xff, 0x18, 0x17, 0x67, 0xff, 0x2c, 0x0e, 0x64, 0xff,
    0x3f, 0x09, 0x4c, 0xff, 0x4f, 0x0d, 0x25, 0xff, 0x51, 0x18, 0x02, 0xff, 0x44, 0x24, 0x00, 0xff,
    0x30, 0x31, 0x00, 0xff, 0x1b, 0x3c, 0x00, 0xff, 0x0b, 0x42, 0x00, 0xff, 0x01, 0x3c, 0x09, 0xff,
    0x00, 0x2f, 0x2f, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xac, 0xac, 0x89, 0xff, 0x3b, 0x5f, 0x8e, 0xff, 0x51, 0x50, 0xa5, 0xff, 0x6a, 0x44, 0xa2, 0xff,
    0x80, 0x3e, 0x87, 0xff, 0x93, 0x44, 0x5a, 0xff, 0x95, 0x52, 0x2c, 0xff, 0x86, 0x60, 0x0d, 0xff,
    0x6e, 0x70, 0x01, 0xff, 0x55, 0x7d, 0x02, 0xff, 0x40, 0x83, 0x12, 0xff, 0x31, 0x7d, 0x36, 0xff,
    0x2f, 0x6e, 0x66, 0xff, 0x1d, 0x1d, 0x11, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xac, 0xac, 0x89, 0xff, 0x7b, 0x8b, 0x8b, 0xff, 0x85, 0x85, 0x94, 0xff, 0x90, 0x7f, 0x93, 0xff,
    0x9a, 0x7d, 0x88, 0xff, 0xa1, 0x7f, 0x75, 0xff, 0xa2, 0x85, 0x60, 0xff, 0x9c, 0x8c, 0x51, 0xff,
    0x92, 0x93, 0x48, 0xff, 0x86, 0x98, 0x49, 0xff, 0x7d, 0x9b, 0x53, 0xff, 0x76, 0x98, 0x65, 0xff,
    0x75, 0x92, 0x7a, 0xff, 0x6c, 0x6c, 0x53, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x37, 0x37, 0x59, 0xff, 0x00, 0x0f, 0x53, 0xff, 0x09, 0x08, 0x68, 0xff, 0x17, 0x00, 0x63, 0xff,
    0x24, 0x00, 0x4b, 0xff, 0x2a, 0x00, 0x28, 0xff, 0x26, 0x00, 0x07, 0xff, 0x1a, 0x03, 0x00, 0xff,
    0x09, 0x0a, 0x00, 0xff, 0x00, 0x13, 0x00, 0xff, 0x00, 0x1a, 0x00, 0xff, 0x00, 0x1b, 0x0e, 0xff,
    0x00, 0x17, 0x31, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x01, 0xff, 0x00, 0x00, 0x01, 0xff,
    0x75, 0x75, 0xab, 0xff, 0x13, 0x37, 0xa1, 0xff, 0x2c, 0x2a, 0xc0, 0xff, 0x43, 0x1b, 0xb8, 0xff,
    0x57, 0x13, 0x94, 0xff, 0x61, 0x12, 0x5f, 0xff, 0x5b, 0x17, 0x2a, 0xff, 0x48, 0x21, 0x06, 0xff,
    0x2c, 0x2d, 0x00, 0xff, 0x17, 0x3d, 0x00, 0xff, 0x08, 0x48, 0x0d, 0xff, 0x03, 0x49, 0x37, 0xff,
    0x06, 0x43, 0x6e, 0xff, 0x00, 0x00, 0x01, 0xff, 0x00, 0x00, 0x01, 0xff, 0x00, 0x00, 0x01, 0xff,
    0xc2, 0xc1, 0xff, 0xff, 0x50, 0x7b, 0xff, 0xff, 0x6e, 0x6c, 0xff, 0xff, 0x89, 0x5a, 0xff, 0xff,
    0xa0, 0x4f, 0xf5, 0xff, 0xab, 0x4d, 0xba, 0xff, 0xa5, 0x54, 0x7b, 0xff, 0x8f, 0x61, 0x4a, 0xff,
    0x6f, 0x6f, 0x30, 0xff, 0x55, 0x83, 0x36, 0xff, 0x40, 0x8e, 0x55, 0xff, 0x38, 0x90, 0x8a, 0xff,
    0x3d, 0x89, 0xca, 0xff, 0x25, 0x25, 0x42, 0xff, 0x00, 0x00, 0x01, 0xff, 0x00, 0x00, 0x01, 0xff,
    0xc2, 0xc1, 0xff, 0xff, 0x91, 0xa4, 0xff, 0xff, 0x9f, 0x9d, 0xff, 0xff, 0xaa, 0x95, 0xff, 0xff,
    0xb4, 0x90, 0xff, 0xff, 0xb9, 0x8f, 0xea, 0xff, 0xb6, 0x92, 0xcf, 0xff, 0xad, 0x98, 0xb8, 0xff,
    0x9f, 0x9f, 0xab, 0xff, 0x93, 0xa7, 0xae, 0xff, 0x8a, 0xac, 0xbd, 0xff, 0x86, 0xad, 0xd6, 0xff,
    0x88, 0xaa, 0xf1, 0xff, 0x7d, 0x7c, 0xb4, 0xff, 0x00, 0x00, 0x01, 0xff, 0x00, 0x00, 0x01, 0xff,
    0x32, 0x27, 0x35, 0xff, 0x00, 0x04, 0x2f, 0xff, 0x05, 0x00, 0x43, 0xff, 0x12, 0x00, 0x41, 0xff,
    0x1e, 0x00, 0x30, 0xff, 0x25, 0x00, 0x17, 0xff, 0x24, 0x00, 0x01, 0xff, 0x18, 0x00, 0x00, 0xff,
    0x08, 0x06, 0x00, 0xff, 0x00, 0x0c, 0x00, 0xff, 0x00, 0x0f, 0x00, 0xff, 0x00, 0x0e, 0x00, 0xff,
    0x00, 0x0a, 0x12, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x6d, 0x5b, 0x73, 0xff, 0x0d, 0x21, 0x69, 0xff, 0x25, 0x16, 0x86, 0xff, 0x3b, 0x0d, 0x84, 0xff,
    0x4f, 0x09, 0x6a, 0xff, 0x5a, 0x0a, 0x43, 0xff, 0x58, 0x10, 0x1b, 0xff, 0x45, 0x1a, 0x00, 0xff,
    0x2a, 0x26, 0x00, 0xff, 0x15, 0x30, 0x00, 0xff, 0x06, 0x36, 0x00, 0xff, 0x00, 0x34, 0x14, 0xff,
    0x01, 0x2c, 0x3b, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xb6, 0x9d, 0xbe, 0xff, 0x45, 0x5a, 0xb4, 0xff, 0x63, 0x4c, 0xd4, 0xff, 0x7d, 0x40, 0xd1, 0xff,
    0x94, 0x3b, 0xb5, 0xff, 0xa0, 0x3c, 0x88, 0xff, 0x9f, 0x45, 0x59, 0xff, 0x89, 0x51, 0x2c, 0xff,
    0x69, 0x5f, 0x15, 0xff, 0x50, 0x6c, 0x16, 0xff, 0x3c, 0x72, 0x2b, 0xff, 0x31, 0x70, 0x51, 0xff,
    0x33, 0x67, 0x7f, 0xff, 0x21, 0x18, 0x24, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0xb6, 0x9d, 0xbe, 0xff, 0x85, 0x81, 0xba, 0xff, 0x93, 0x7a, 0xc7, 0xff, 0x9e, 0x75, 0xc6, 0xff,
    0xa8, 0x72, 0xba, 0xff, 0xad, 0x73, 0xa8, 0xff, 0xac, 0x77, 0x93, 0xff, 0xa3, 0x7d, 0x7e, 0xff,
    0x95, 0x83, 0x71, 0xff, 0x8a, 0x88, 0x72, 0xff, 0x81, 0x8b, 0x7d, 0xff, 0x7c, 0x8a, 0x8f, 0xff,
    0x7c, 0x86, 0xa3, 0xff, 0x73, 0x62, 0x7a, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x24, 0x2f, 0x30, 0xff, 0x00, 0x0a, 0x2f, 0xff, 0x00, 0x03, 0x3d, 0xff, 0x07, 0x00, 0x39, 0xff,
    0x12, 0x00, 0x23, 0xff, 0x19, 0x00, 0x0c, 0xff, 0x18, 0x00, 0x00, 0xff, 0x10, 0x01, 0x00, 0xff,
    0x05, 0x08, 0x00, 0xff, 0x00, 0x11, 0x00, 0xff, 0x00, 0x17, 0x00, 0xff, 0x00, 0x16, 0x01, 0xff,
    0x00, 0x11, 0x17, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x57, 0x68, 0x6a, 0xff, 0x04, 0x2d, 0x68, 0xff, 0x14, 0x20, 0x7e, 0xff, 0x28, 0x12, 0x77, 0xff,
    0x3b, 0x0a, 0x56, 0xff, 0x45, 0x0b, 0x30, 0xff, 0x44, 0x12, 0x0c, 0xff, 0x37, 0x1d, 0x00, 0xff,
    0x23, 0x29, 0x00, 0xff, 0x0f, 0x39, 0x00, 0xff, 0x02, 0x44, 0x01, 0xff, 0x00, 0x42, 0x1c, 0xff,
    0x00, 0x39, 0x43, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x97, 0xaf, 0xb2, 0xff, 0x33, 0x6b, 0xb0, 0xff, 0x48, 0x5c, 0xc8, 0xff, 0x61, 0x4a, 0xc0, 0xff,
    0x77, 0x3f, 0x9b, 0xff, 0x83, 0x41, 0x70, 0xff, 0x81, 0x4a, 0x43, 0xff, 0x72, 0x58, 0x21, 0xff,
    0x5b, 0x67, 0x11, 0xff, 0x42, 0x7a, 0x16, 0xff, 0x2f, 0x86, 0x31, 0xff, 0x25, 0x84, 0x57, 0xff,
    0x27, 0x7a, 0x86, 0xff, 0x16, 0x1e, 0x20, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x97, 0xaf, 0xb2, 0xff, 0x6b, 0x92, 0xb1, 0xff, 0x75, 0x8c, 0xbb, 0xff, 0x80, 0x83, 0xb8, 0xff,
    0x89, 0x7f, 0xa9, 0xff, 0x8e, 0x7f, 0x96, 0xff, 0x8e, 0x83, 0x82, 0xff, 0x88, 0x8a, 0x72, 0xff,
    0x7e, 0x91, 0x69, 0xff, 0x73, 0x99, 0x6c, 0xff, 0x6a, 0x9e, 0x7a, 0xff, 0x65, 0x9d, 0x8b, 0xff,
    0x65, 0x99, 0xa0, 0xff, 0x5d, 0x6e, 0x71, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x25, 0x25, 0x25, 0xff, 0x00, 0x03, 0x23, 0xff, 0x00, 0x00, 0x31, 0xff, 0x07, 0x00, 0x2f, 0xff,
    0x12, 0x00, 0x20, 0xff, 0x19, 0x00, 0x09, 0xff, 0x18, 0x00, 0x00, 0xff, 0x10, 0x00, 0x00, 0xff,
    0x05, 0x05, 0x00, 0xff, 0x00, 0x0b, 0x00, 0xff, 0x00, 0x0f, 0x00, 0xff, 0x00, 0x0e, 0x00, 0xff,
    0x00, 0x09, 0x0d, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x58, 0x58, 0x58, 0xff, 0x05, 0x1f, 0x56, 0xff, 0x14, 0x13, 0x6b, 0xff, 0x28, 0x0a, 0x68, 0xff,
    0x3b, 0x06, 0x50, 0xff, 0x45, 0x07, 0x2b, 0xff, 0x44, 0x0e, 0x08, 0xff, 0x37, 0x18, 0x00, 0xff,
    0x24, 0x24, 0x00, 0xff, 0x10, 0x2f, 0x00, 0xff, 0x02, 0x35, 0x00, 0xff, 0x00, 0x33, 0x0e, 0xff,
    0x00, 0x2b, 0x32, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x98, 0x98, 0x98, 0xff, 0x33, 0x56, 0x96, 0xff, 0x48, 0x47, 0xac, 0xff, 0x61, 0x3c, 0xaa, 0xff,
    0x77, 0x36, 0x8f, 0xff, 0x83, 0x38, 0x64, 0xff, 0x81, 0x40, 0x38, 0xff, 0x73, 0x4e, 0x18, 0xff,
    0x5c, 0x5d, 0x09, 0xff, 0x43, 0x69, 0x0a, 0xff, 0x30, 0x70, 0x1d, 0xff, 0x26, 0x6e, 0x40, 0xff,
    0x27, 0x64, 0x6d, 0xff, 0x16, 0x16, 0x16, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x98, 0x98, 0x98, 0xff, 0x6c, 0x7c, 0x97, 0xff, 0x76, 0x75, 0xa0, 0xff, 0x81, 0x70, 0x9f, 0xff,
    0x8a, 0x6d, 0x94, 0xff, 0x8f, 0x6e, 0x82, 0xff, 0x8e, 0x72, 0x6e, 0xff, 0x88, 0x78, 0x5e, 0xff,
    0x7e, 0x7f, 0x56, 0xff, 0x73, 0x84, 0x57, 0xff, 0x6a, 0x87, 0x61, 0xff, 0x65, 0x86, 0x72, 0xff,
    0x66, 0x82, 0x86, 0xff, 0x5e, 0x5e, 0x5e, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
];
