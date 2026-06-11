use crate::graphics::types::GColor8;

impl GColor8 {
    // Basic Grayscale & Clear
    pub const CLEAR: Self = Self::new(0b0000_0000);
    pub const BLACK: Self = Self::new(0b1100_0000);
    pub const DARK_GRAY: Self = Self::new(0b1101_0101);
    pub const LIGHT_GRAY: Self = Self::new(0b1110_1010);
    pub const WHITE: Self = Self::new(0b1111_1111);

    // Blues
    pub const OXFORD_BLUE: Self = Self::new(0b1100_0001);
    pub const DUKE_BLUE: Self = Self::new(0b1100_0010);
    pub const BLUE: Self = Self::new(0b1100_0011);
    pub const COBALT_BLUE: Self = Self::new(0b1100_0110);
    pub const BLUE_MOON: Self = Self::new(0b1100_0111);
    pub const VIVID_CERULEAN: Self = Self::new(0b1100_1011);
    pub const ELECTRIC_ULTRAMARINE: Self = Self::new(0b1101_0011);
    pub const LIBERTY: Self = Self::new(0b1101_0110);
    pub const VERY_LIGHT_BLUE: Self = Self::new(0b1101_0111);
    pub const CADET_BLUE: Self = Self::new(0b1101_1010);
    pub const PICTON_BLUE: Self = Self::new(0b1101_1011);
    pub const ELECTRIC_BLUE: Self = Self::new(0b1101_1111);
    pub const BABY_BLUE_EYES: Self = Self::new(0b1110_1011);

    // Greens
    pub const DARK_GREEN: Self = Self::new(0b1100_0100);
    pub const MIDNIGHT_GREEN: Self = Self::new(0b1100_0101);
    pub const ISLAMIC_GREEN: Self = Self::new(0b1100_1000);
    pub const JAEGER_GREEN: Self = Self::new(0b1100_1001);
    pub const TIFFANY_BLUE: Self = Self::new(0b1100_1010);
    pub const GREEN: Self = Self::new(0b1100_1100);
    pub const MALACHITE: Self = Self::new(0b1100_1101);
    pub const MEDIUM_SPRING_GREEN: Self = Self::new(0b1100_1110);
    pub const CYAN: Self = Self::new(0b1100_1111);
    pub const ARMY_GREEN: Self = Self::new(0b1101_0100);
    pub const KELLY_GREEN: Self = Self::new(0b1101_1000);
    pub const MAY_GREEN: Self = Self::new(0b1101_1001);
    pub const BRIGHT_GREEN: Self = Self::new(0b1101_1100);
    pub const SCREAMIN_GREEN: Self = Self::new(0b1101_1101);
    pub const MEDIUM_AQUAMARINE: Self = Self::new(0b1101_1110);
    pub const LIMERICK: Self = Self::new(0b1110_1000);
    pub const SPRING_BUD: Self = Self::new(0b1110_1100);
    pub const INCHWORM: Self = Self::new(0b1110_1101);
    pub const MINT_GREEN: Self = Self::new(0b1110_1110);
    pub const CELESTE: Self = Self::new(0b1110_1111);

    // Reds & Pinks
    pub const BULGARIAN_ROSE: Self = Self::new(0b1101_0000);
    pub const DARK_CANDY_APPLE_RED: Self = Self::new(0b1110_0000);
    pub const JAZZBERRY_JAM: Self = Self::new(0b1110_0001);
    pub const ROSE_VALE: Self = Self::new(0b1110_0101);
    pub const RED: Self = Self::new(0b1111_0000);
    pub const FOLLY: Self = Self::new(0b1111_0001);
    pub const FASHION_MAGENTA: Self = Self::new(0b1111_0010);
    pub const MAGENTA: Self = Self::new(0b1111_0011);
    pub const SUNSET_ORANGE: Self = Self::new(0b1111_0101);
    pub const BRILLIANT_ROSE: Self = Self::new(0b1111_0110);
    pub const SHOCKING_PINK: Self = Self::new(0b1111_0111);

    // Purples
    pub const IMPERIAL_PURPLE: Self = Self::new(0b1101_0001);
    pub const INDIGO: Self = Self::new(0b1101_0010);
    pub const PURPLE: Self = Self::new(0b1110_0010);
    pub const VIVID_VIOLET: Self = Self::new(0b1110_0011);
    pub const PURPUREUS: Self = Self::new(0b1110_0110);
    pub const LAVENDER_INDIGO: Self = Self::new(0b1110_0111);
    pub const RICH_BRILLIANT_LAVENDER: Self = Self::new(0b1111_1011);

    // Yellows & Oranges
    pub const WINDSOR_TAN: Self = Self::new(0b1110_0100);
    pub const BRASS: Self = Self::new(0b1110_1001);
    pub const ORANGE: Self = Self::new(0b1111_0100);
    pub const CHROME_YELLOW: Self = Self::new(0b1111_1000);
    pub const RAJAH: Self = Self::new(0b1111_1001);
    pub const MELON: Self = Self::new(0b1111_1010);
    pub const YELLOW: Self = Self::new(0b1111_1100);
    pub const ICTERINE: Self = Self::new(0b1111_1101);
    pub const PASTEL_YELLOW: Self = Self::new(0b1111_1110);
}