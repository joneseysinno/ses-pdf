use crate::element::Equation;

pub(crate) fn generate_equation(eq: &Equation) -> String {
    let label = eq
        .label
        .as_deref()
        .map(|l| format!(" <{}>", l))
        .unwrap_or_default();

    if eq.display {
        format!("\n$ {} ${}\n", eq.math, label)
    } else {
        format!("${}${}", eq.math, label)
    }
}

