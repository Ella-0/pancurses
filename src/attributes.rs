use std::ops::BitOr;
use super::{chtype, A_ALTCHARSET, A_BOLD, A_BLINK, A_CHARTEXT, A_DIM, A_LEFTLINE, A_INVIS};
use super::{A_ITALIC, A_OVERLINE, A_REVERSE, A_RIGHTLINE, A_STRIKEOUT, A_UNDERLINE};
use super::{COLOR_PAIR};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Attribute {
   AlternativeCharSet,
   Bold,
   Blink,
   CharText,
   Dim,
   Leftline,
   Invisible,
   Italic,
   Normal,
   Overline,
   Reverse,
   Rightline,
   Strikeout,
   Underline,
   ColorPair(chtype)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attributes {
   raw: chtype,
   color_pair: chtype
}

macro_rules! attribute_setter {
    ($name:ident, $attr:ident) => {
        pub fn $name(&mut self, enabled: bool) {
            if enabled {
                self.raw = self.raw | $attr;
            } else {
                self.raw = self.raw ^ $attr;
            }
        }
    };
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            raw: 0,
            color_pair: 0
        }
    }

    pub fn is_alternative_char_set(&self) -> bool { (self.raw & A_ALTCHARSET) > 0 }
    attribute_setter!(set_alternative_char_set, A_ALTCHARSET);

    pub fn is_bold(&self) -> bool { (self.raw & A_BOLD) > 0 }
    attribute_setter!(set_bold, A_BOLD);

    pub fn is_blink(&self) -> bool { (self.raw & A_BLINK) > 0 }
    attribute_setter!(set_blink, A_BLINK);

    pub fn is_char_text(&self) -> bool { (self.raw & A_CHARTEXT) > 0 }
    attribute_setter!(set_char_text, A_CHARTEXT);

    pub fn is_dim(&self) -> bool { (self.raw & A_DIM) > 0 }
    attribute_setter!(set_dim, A_DIM);

    pub fn is_leftline(&self) -> bool { (self.raw & A_LEFTLINE) > 0 }
    attribute_setter!(set_leftline, A_LEFTLINE);

    pub fn is_invisible(&self) -> bool { (self.raw & A_INVIS) > 0 } 
    attribute_setter!(set_invisible, A_INVIS);

    pub fn is_italic(&self) -> bool { (self.raw & A_ITALIC) > 0 }
    attribute_setter!(set_italic, A_ITALIC);

    pub fn is_normal(&self) -> bool { self.raw == 0 }
    pub fn set_normal(&mut self) { self.raw = 0 }

    pub fn is_overline(&self) -> bool { (self.raw & A_OVERLINE) > 0 }
    attribute_setter!(set_overline, A_OVERLINE);

    pub fn is_reverse(&self) -> bool { (self.raw & A_REVERSE) > 0 }
    attribute_setter!(set_reverse, A_REVERSE);

    pub fn is_rightline(&self) -> bool { (self.raw & A_RIGHTLINE) > 0 }
    attribute_setter!(set_rightline, A_RIGHTLINE);

    pub fn is_strikeout(&self) -> bool { (self.raw & A_STRIKEOUT) > 0 }
    attribute_setter!(set_strikeout, A_STRIKEOUT);

    pub fn is_underline(&self) -> bool { (self.raw & A_UNDERLINE) > 0 }
    attribute_setter!(set_underline, A_UNDERLINE);

    pub fn color_pair(&self) -> chtype { self.color_pair }
    pub fn set_color_pair(&mut self, color_pair: chtype) { 
        self.raw = self.raw | COLOR_PAIR(color_pair);
        self.color_pair = color_pair;
    }
}

/// Implement the | operator for adding an Attribute to Attributes
///
/// # Example
///
/// ```
/// use pancurses::{Attribute, Attributes};
///
/// let mut attributes = Attributes::new();
/// assert!(!attributes.is_bold());
/// attributes = attributes | Attribute::Bold;
/// assert!(attributes.is_bold());
/// ```
impl BitOr<Attribute> for Attributes {
    type Output = Attributes;

    fn bitor(mut self, rhs: Attribute) -> Attributes {
        match rhs {
            Attribute::AlternativeCharSet => self.set_alternative_char_set(true),
            Attribute::Bold => self.set_bold(true),
            Attribute::Blink => self.set_blink(true),
            Attribute::CharText => self.set_char_text(true),
            Attribute::Dim => self.set_dim(true),
            Attribute::Leftline => self.set_leftline(true),
            Attribute::Invisible => self.set_invisible(true),
            Attribute::Italic => self.set_italic(true),
            Attribute::Normal => self.set_normal(),
            Attribute::Overline => self.set_overline(true),
            Attribute::Reverse => self.set_reverse(true),
            Attribute::Rightline => self.set_rightline(true),
            Attribute::Strikeout => self.set_strikeout(true),
            Attribute::Underline => self.set_underline(true),
            Attribute::ColorPair(num) => self.set_color_pair(num)
        }
        self
    }
}

/// Implement the | operator for adding Attributes to Attributes
///
/// # Example
///
/// ```
/// use pancurses::{Attribute, Attributes};
///
/// let mut attributes = Attributes::new() | Attribute::Bold;
/// let other = Attributes::new() | Attribute::Reverse;
/// attributes = attributes | other;
/// assert!(attributes.is_bold());
/// assert!(attributes.is_reverse());
/// assert!(!attributes.is_italic());
/// ```
impl BitOr for Attributes {
    type Output = Attributes;

    fn bitor(self, rhs: Attributes) -> Attributes {
        Attributes{
            raw: self.raw | rhs.raw,
            color_pair: self.color_pair | rhs.color_pair
        }
    }
}

/// Implement the | operator for combining two 'Attribute's into Attributes
///
/// # Example
///
/// ```
/// use pancurses::{Attribute, Attributes};
///
/// let attributes = Attribute::Blink | Attribute::Reverse;
/// assert!(!attributes.is_bold());
/// assert!(attributes.is_blink());
/// assert!(attributes.is_reverse());
/// ```
impl BitOr for Attribute {
    type Output = Attributes;

    fn bitor(self, rhs: Attribute) -> Attributes {
        Attributes::new() | self | rhs
    }
}

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Attributes {
        Attributes::new() | attribute
    }
}

impl From<Attribute> for chtype {
    fn from(attribute: Attribute) -> chtype {
        chtype::from(Attributes::from(attribute))
    }
}

impl From<Attributes> for chtype {
    fn from(attributes: Attributes) -> chtype {
        attributes.raw
    }
}