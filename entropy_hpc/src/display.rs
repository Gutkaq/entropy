// src/display.rs

use crate::cint::{CInt, CIFraction};
use crate::hint::{HInt, HIFraction};
use crate::oint::{OInt, OIFraction};
use std::fmt;

// ========================================================================
// CINT (Complex Integers) Display
// ========================================================================

impl fmt::Display for CInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.a, self.b)
    }
}

impl fmt::Display for CIFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) / {}", self.num, self.den)
    }
}

// ========================================================================
// HINT (Hurwitz Quaternions) Display
// ========================================================================

impl fmt::Display for HInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (a, b, c, d) = self.to_float_components();
        
        write!(f, "{}", format_component(a, "", true))?;
        write!(f, "{}", format_component(b, "i", false))?;
        write!(f, "{}", format_component(c, "j", false))?;
        write!(f, "{}", format_component(d, "k", false))
    }
}

impl fmt::Display for HIFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (a, b, c, d) = self.num.to_float_components();
        
        write!(f, "(")?;
        write!(f, "{}", format_component(a, "", true))?;
        write!(f, "{}", format_component(b, "i", false))?;
        write!(f, "{}", format_component(c, "j", false))?;
        write!(f, "{}", format_component(d, "k", false))?;
        write!(f, ") / {}", self.den)
    }
}

// ========================================================================
// OINT (Integer Octonions) Display
// ========================================================================

impl fmt::Display for OInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (a, b, c, d, e, f_val, g, h) = self.to_float_components();
        
        write!(f, "{}", format_component(a, "", true))?;
        write!(f, "{}", format_component(b, "e₁", false))?;
        write!(f, "{}", format_component(c, "e₂", false))?;
        write!(f, "{}", format_component(d, "e₃", false))?;
        write!(f, "{}", format_component(e, "e₄", false))?;
        write!(f, "{}", format_component(f_val, "e₅", false))?;
        write!(f, "{}", format_component(g, "e₆", false))?;
        write!(f, "{}", format_component(h, "e₇", false))
    }
}

impl fmt::Display for OIFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (a, b, c, d, e, f_val, g, h) = self.num.to_float_components();
        
        write!(f, "(")?;
        write!(f, "{}", format_component(a, "", true))?;
        write!(f, "{}", format_component(b, "e₁", false))?;
        write!(f, "{}", format_component(c, "e₂", false))?;
        write!(f, "{}", format_component(d, "e₃", false))?;
        write!(f, "{}", format_component(e, "e₄", false))?;
        write!(f, "{}", format_component(f_val, "e₅", false))?;
        write!(f, "{}", format_component(g, "e₆", false))?;
        write!(f, "{}", format_component(h, "e₇", false))?;
        write!(f, ") / {}", self.den)
    }
}

// ========================================================================
// Helper function for formatting components
// ========================================================================

fn format_component(val: f64, unit: &str, is_first: bool) -> String {
    // Handle zero components
    if val == 0.0 && !is_first {
        return String::new();
    }
    
    // Determine if we need to show as fraction
    let is_fraction = val.fract() != 0.0;
    
    // Format the number
    let num_str = if is_fraction {
        if (val.fract().abs() - 0.5).abs() < 0.0001 {
            // It's a .5 fraction
            let whole = val.trunc() as i32;
            if whole == 0 {
                "1/2".to_string()
            } else {
                format!("{} + 1/2", whole)
            }
        } else {
            format!("{}", val)
        }
    } else {
        format!("{}", val as i32)
    };
    
    // Build the term
    if is_first {
        if unit.is_empty() {
            num_str
        } else {
            format!("{}{}", num_str, unit)
        }
    } else {
        let abs_val = val.abs();
        let abs_is_fraction = abs_val.fract() != 0.0;
        
        let abs_str = if abs_is_fraction {
            if (abs_val.fract().abs() - 0.5).abs() < 0.0001 {
                let whole = abs_val.trunc() as i32;
                if whole == 0 {
                    "1/2".to_string()
                } else {
                    format!("{} + 1/2", whole)
                }
            } else {
                format!("{}", abs_val)
            }
        } else {
            format!("{}", abs_val as i32)
        };
        
        let sign = if val >= 0.0 { " + " } else { " - " };
        
        if unit.is_empty() {
            format!("{}{}", sign, abs_str)
        } else {
            format!("{}{}{}", sign, abs_str, unit)
        }
    }
}

// ========================================================================
// Debug implementations (delegate to Display)
// ========================================================================

impl fmt::Debug for CInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CInt({})", self)
    }
}

impl fmt::Debug for CIFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CIFraction({})", self)
    }
}

impl fmt::Debug for HInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HInt({})", self)
    }
}

impl fmt::Debug for HIFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HIFraction({})", self)
    }
}

impl fmt::Debug for OInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OInt({})", self)
    }
}

impl fmt::Debug for OIFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OIFraction({})", self)
    }
}

