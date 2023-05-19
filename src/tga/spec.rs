use std::fs::File;
use std::io::Write;

pub const TGA_SIGNATURE: [u8; 18] = *b"TRUEVISION-XFILE.\0";
#[warn(dead_code)]
pub struct TgaHeader {
    /*
    ID Length - Field 1 (1 byte)

    This field identifies the number of bytes contained in Field 6, the Image ID Field. The maximum number  of characters is 255. A value of zero indicates that no Image ID field is included with the image.
    */
    pub idlength: u8,

    /*
    Color Map Type - Field 2 (1 byte)

    This field indicates the type of color map (if any) included with the image. There are currently 2 defined  values for this field:
    0 - indicates that no color-map data is included with this image.
    1 - indicates that a color-map is included with this image.

    The first 128 Color Map Type codes (Field 2) are reserved for use by Truevision, while the second set of  128 Color Map Type codes (128 to 255) may be used for developer applications.

    True-Color images do not normally make use of the color map field, but some current applications store  palette information or developer-defined information in this field. It is best to check Field 3, Image Type,  to make sure you have a file which can use the data stored in the Color Map Field. Otherwise ignore the  information. When saving or creating files for True-Color images do not use this field and set it to Zero to  ensure compatibility. Please refer to the Developer Area specification for methods of storing developer  defined information.
    */
    pub colormaptype: u8,

    /*
    Image Type - Field 3 (1 byte)

    The TGA File Format can be used to store Pseudo-Color, True-Color and Direct-Color images of various  pixel depths. Truevision has currently defined seven image types:

    0 - No Image Data is present.
    1 - Uncompressed, Color-Mapped Images.
    2 - Uncompressed, True-Color Images.
    3 - Uncompressed, Black & White Images.
    9 - Runlength Encoded Color-Mapped Images.
    10 - Runlength Encoded True-Color Images.
    11 - Compressed, Black & White Images.

    Image Data Type codes 0 to 127 are reserved for use by Truevision for general applications. Image Data  Type codes 128 to 255 may be used for developer applications.
    */
    pub datatypecode: u8,

    /*
    Color Map Specification - Field 4 (5 bytes)

    This field and its sub-fields describe the color map (if any) used for the image. If the Color Map Type field  is set to zero, indicating that no color map exists, then these 5 bytes should be set to zero. These bytes  always must be written to the file.

    Color Map Origin - Field 4.1 (2 bytes) - First Entry Index:
    Index of the first color map entry. Index refers to the starting entry in  loading the color map.
    */
    pub colormaporigin: u16,

    /*
    Color Map Length - Field 4.2 (2 bytes) - Color Map Length:
    Total number of color map entries included.
    */
    pub colormaplength: u16,

    /*
    Color Map Entry Size - Field 4.3 (1 byte) - Color Map Entry Size:
    Number of bits in each color map entry. This is 16 for the Targa 16, 24 for the Targa 24, and 32 for the  Targa 32. This field is used to determine the amount of information in the Color Map Field.
    */
    pub colormapdepth: u8,

    /*
    Image Specification - Field 5 (10 bytes)
    This field and its sub-fields describe the image screen location, size and pixel depth. These bytes are  always written to the file.

    Field 5.1 (2 bytes) -  X-origin of Image:

    These bytes specify the absolute horizontal coordinate for the lower left  corner of the image as it is positioned on a display device having an  origin at the lower left of the screen (e.g., the TARGA series).
    */
    pub x_origin: u16,

    /*
    Field 5.2 (2 bytes) -  Y-origin of Image:

    These bytes specify the absolute vertical coordinate for the lower left  corner of the image as it is positioned on a display device having an  origin at the lower left of the screen (e.g., the TARGA series)
    */
    pub y_origin: u16,

    /*
    Field 5.3 (2 bytes) -  Width of Image:

    This field specifies the width of the image in pixels.
    */
    pub width: u16,

    /*
    Field 5.4 (2 bytes) -  Height of Image:

    This field specifies the height of the image in pixels.
    */
    pub height: u16,

    /*
    Field 5.5 (1 byte) -  Pixel Depth:
    This field indicates the number of bits per pixel. This number includes  the Attribute or Alpha channel bits. Common values are 8, 16, 24 and  32 but other pixel depths could be used.
    */
    pub bitsperpixel: u8,

    /*
    Field 5.6 (1 byte) -  Image Descriptor:

    Bits 3-0:  Attribute Bits - (For Targa 16 and 32 Only)

    These bits specify the number of attribute bits per  pixel. In the case of the TrueVista, these bits indicate the number of bits per pixel which are designated as Alpha Channel bits. For the ICB and TARGA products, these bits indicate the number of overlay bits available per pixel. See  field 24 (Attributes Type) for more information.

    Bits 5 & 4:

    These bits are used to indicate the order in which  pixel data is transferred from the file to the screen.  Bit 4 is for left-to-right ordering and bit 5 is for  top-to-bottom ordering as shown below.

    Screen destination of first pixel   Bit 5   Bit 4
    Bottom left-hand corner               0       0
    Bottom right-hand corner              0       1
    Top left-hand corner                  1       0
    Top right-hand corner                 1       1

    Bits 7 & 6:  Must be zero to insure future compatibility

    Binary      Hex     Screen Origin
    00000000    0x00    Bottom Left
    00010000    0x10    Bottom Right
    00100000    0x20    Top Left
    00110000    0x30    Top Right

    */
    pub imagedescriptor: u8,
}

impl TgaHeader {
    pub fn new() -> Self {
        TgaHeader {
            idlength: 0,
            colormaptype: 0,
            datatypecode: 0,
            colormaporigin: 0,
            colormaplength: 0,
            colormapdepth: 0,
            x_origin: 0,
            y_origin: 0,
            width: 0,
            height: 0,
            bitsperpixel: 0,
            imagedescriptor: 0x20,
        }
    }

    pub fn write(&self, file: &mut File) -> Result<(), String> {
        file.write_all(&self.idlength.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.colormaptype.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.datatypecode.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.colormaporigin.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.colormaplength.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.colormapdepth.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.x_origin.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.y_origin.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.width.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.height.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.bitsperpixel.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.imagedescriptor.to_le_bytes())
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn read(&mut self, buffer: &Vec<u8>, offset: &mut usize) -> Result<(), String> {
        self.idlength = read_u8(buffer, offset);
        self.colormaptype = read_u8(buffer, offset);
        self.datatypecode = read_u8(buffer, offset);
        self.colormaporigin = read_u16(buffer, offset);
        self.colormaplength = read_u16(buffer, offset);
        self.colormapdepth = read_u8(buffer, offset);
        self.x_origin = read_u16(buffer, offset);
        self.y_origin = read_u16(buffer, offset);
        self.width = read_u16(buffer, offset);
        self.height = read_u16(buffer, offset);
        self.bitsperpixel = read_u8(buffer, offset);
        self.imagedescriptor = read_u8(buffer, offset);
        Ok(())
    }
}

pub struct TgaFooter {
    /*
    1.  A TGA Reader should begin by determining whether the desired file is in the Original TGA  Format or the New TGA Format. This is accomplished by examining the last 26 bytes of the  file (most operating systems support some type of SEEK function). Reading the last 26 bytes  from the file will either retrieve the last 26 bytes of image data (if the file is in the Original  TGA Format), or it will retrieve the TGA File Footer (if the file is in the New TGA Format).
    2.  To determine whether the acquired data constitutes a legal TGA File Footer, scan bytes 8-23  of the footer as ASCII characters and determine whether they match the signature string:  TRUEVISION-XFILE

    This string is exactly 16 bytes long and is formatted exactly as shown above (capital letters), with a hyphen between “TRUEVISION” and “XFILE.” If the signature is detected, the file  is assumed to be in the New TGA format and MAY, therefore, contain the Developer Area  and/or the Extension Area fields. If the signature is not found, then the file is assumed to be  in the Original TGA format and should only contain areas 1 and 2; therefore, the byte format  for the TGA File Footer is defined as follows:

    Bytes 0-3: The Extension Area Offset
    Bytes 4-7: The Developer Directory Offset
    Bytes 8-23: The Signature
    Byte 24: ASCII Character "."
    Byte 25: Binary zero string terminator (0x00)
    */
    /*
    Byte 0-3 - Extension Area Offset - Field 28
    The first four bytes (bytes 0-3, the first LONG) of the TGA File Footer contain an offset from the beginning of the file to the start of the Extension Area. Simply SEEK to this location to position to the start of the Extension Area. If the Extension Area Offset is zero, no Extension Area exists in the file.
    */
    pub extensionareaoffset: u32,

    /*
    Byte 4-7 - Developer Directory Offset - Field 29
    The next four bytes (bytes 4-7, the second LONG) contain an offset from the  beginning of the file to the start of the Developer Directory. If the Developer Directory Offset is zero, then the Developer Area does not exist.
    */
    pub developerdirectoryoffset: u32,

    /*
    Byte 8-23 - Signature - Field 30
    This string is exactly 16 bytes long and is formatted exactly as shown below capital  letters), with a hyphen between “TRUEVISION” and “XFILE.” If the signature is  detected, the file is assumed to be of the New TGA format and MAY, therefore,  contain the Developer Area and/or the Extension Area fields. If the signature is not  found, then the file is assumed to be in the Original TGA format.

    TRUEVISION-XFILE

    Byte 24 - Reserved Character - Field 31
    Byte 24 is an ASCII character “.” (period). This character MUST BE a period or the  file is not considered a proper TGA file.

    Byte 25 - Binary Zero String Terminator - Field 32
    Byte 25 is a binary zero which acts as a final terminator and allows the entire TGA  File Footer to be read and utilized as a “C” string
    */
    signature: [u8; 18],
}

impl TgaFooter {
    pub fn new() -> Self {
        TgaFooter {
            extensionareaoffset: 0,
            developerdirectoryoffset: 0,
            signature: TGA_SIGNATURE,
        }
    }

    pub fn write(&self, file: &mut File) -> Result<(), String> {
        file.write_all(&self.extensionareaoffset.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.developerdirectoryoffset.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&self.signature).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn read(&mut self, buffer: &Vec<u8>) -> Result<(), String> {
        let mut offset = buffer.len() - 26;

        self.extensionareaoffset = read_u32(&buffer, &mut offset);
        self.developerdirectoryoffset = read_u32(&buffer, &mut offset);
        self.signature = [
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
            read_u8(buffer, &mut offset),
        ];

        match self.is_valid() {
            false => return Err("Invalid TGA footer".to_string()),
            _ => (),
        }

        Ok(())
    }

    fn is_valid(&self) -> bool {
        self.signature == TGA_SIGNATURE
    }
}

fn read_u8(buffer: &Vec<u8>, offset: &mut usize) -> u8 {
    let value = u8::from_le_bytes([buffer[*offset]]);
    *offset += 1;
    value
}

fn read_u16(buffer: &Vec<u8>, offset: &mut usize) -> u16 {
    let value = u16::from_le_bytes([buffer[*offset], buffer[*offset + 1]]);
    *offset += 2;
    value
}

fn read_u32(buffer: &Vec<u8>, offset: &mut usize) -> u32 {
    let value = u32::from_le_bytes([
        buffer[*offset],
        buffer[*offset + 1],
        buffer[*offset + 2],
        buffer[*offset + 3],
    ]);
    *offset += 4;
    value
}
