use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::config::TemplateType;

macro_rules! include_template {
    ($path:expr) => {
        include_str!($path)
    };
}

// Static lazy-initialized maps for subjects and templates
static SUBJECTS: Lazy<HashMap<&'static str, HashMap<TemplateType, &'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Define subjects for different locales
    let en_subjects = HashMap::from([
        (TemplateType::Invite, "You've been invited to join"),
        (TemplateType::Confirmation, "Please confirm your email address"),
        (TemplateType::Recovery, "Reset your password"),
        (TemplateType::MagicLink, "Sign in to your account"),
        (TemplateType::EmailChange, "Confirm your email address change"),
    ]);
    let es_subjects = HashMap::from([
        (TemplateType::Invite, "Has sido invitado a unirte"),
        (TemplateType::Confirmation, "Por favor, confirma tu dirección de correo electrónico"),
        (TemplateType::Recovery, "Restablecer tu contraseña"),
        (TemplateType::MagicLink, "Inicia sesión en tu cuenta"),
        (TemplateType::EmailChange, "Confirma el cambio de tu dirección de correo electrónico"),
    ]);
    let fr_subjects = HashMap::from([
        (TemplateType::Invite, "Vous avez été invité à rejoindre"),
        (TemplateType::Confirmation, "Veuillez confirmer votre adresse e-mail"),
        (TemplateType::Recovery, "Réinitialisez votre mot de passe"),
        (TemplateType::MagicLink, "Connectez-vous à votre compte"),
        (TemplateType::EmailChange, "Confirmez le changement de votre adresse e-mail"),
    ]);

    map.insert("en", en_subjects);
    map.insert("es", es_subjects);
    map.insert("fr", fr_subjects);

    map
});

static TEMPLATES: Lazy<HashMap<&'static str, HashMap<TemplateType, (&'static str, &'static str)>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Define templates for different locales
    let en_templates = HashMap::from([
        (TemplateType::Invite, (include_template!("../../templates/mailer/invite/en.txt.hbs"), include_template!("../../templates/mailer/invite/en.html.hbs"))),
        (TemplateType::Confirmation, (include_template!("../../templates/mailer/confirmation/en.txt.hbs"), include_template!("../../templates/mailer/confirmation/en.html.hbs"))),
        (TemplateType::Recovery, (include_template!("../../templates/mailer/recovery/en.txt.hbs"), include_template!("../../templates/mailer/recovery/en.html.hbs"))),
        (TemplateType::MagicLink, (include_template!("../../templates/mailer/magic_link/en.txt.hbs"), include_template!("../../templates/mailer/magic_link/en.html.hbs"))),
        (TemplateType::EmailChange, (include_template!("../../templates/mailer/email_change/en.txt.hbs"), include_template!("../../templates/mailer/email_change/en.html.hbs"))),
    ]);
    let es_templates = HashMap::from([
        (TemplateType::Invite, (include_template!("../../templates/mailer/invite/es.txt.hbs"), include_template!("../../templates/mailer/invite/es.html.hbs"))),
        (TemplateType::Confirmation, (include_template!("../../templates/mailer/confirmation/es.txt.hbs"), include_template!("../../templates/mailer/confirmation/es.html.hbs"))),
        (TemplateType::Recovery, (include_template!("../../templates/mailer/recovery/es.txt.hbs"), include_template!("../../templates/mailer/recovery/es.html.hbs"))),
        (TemplateType::MagicLink, (include_template!("../../templates/mailer/magic_link/es.txt.hbs"), include_template!("../../templates/mailer/magic_link/es.html.hbs"))),
        (TemplateType::EmailChange, (include_template!("../../templates/mailer/email_change/es.txt.hbs"), include_template!("../../templates/mailer/email_change/es.html.hbs"))),
    ]);
    let fr_templates = HashMap::from([
        (TemplateType::Invite, (include_template!("../../templates/mailer/invite/fr.txt.hbs"), include_template!("../../templates/mailer/invite/fr.html.hbs"))),
        (TemplateType::Confirmation, (include_template!("../../templates/mailer/confirmation/fr.txt.hbs"), include_template!("../../templates/mailer/confirmation/fr.html.hbs"))),
        (TemplateType::Recovery, (include_template!("../../templates/mailer/recovery/fr.txt.hbs"), include_template!("../../templates/mailer/recovery/fr.html.hbs"))),
        (TemplateType::MagicLink, (include_template!("../../templates/mailer/magic_link/fr.txt.hbs"), include_template!("../../templates/mailer/magic_link/fr.html.hbs"))),
        (TemplateType::EmailChange, (include_template!("../../templates/mailer/email_change/fr.txt.hbs"), include_template!("../../templates/mailer/email_change/fr.html.hbs"))),
    ]);

    map.insert("en", en_templates);
    map.insert("es", es_templates);
    map.insert("fr", fr_templates);

    map
});

pub fn get_fallback_subject(locale: &str, template_type: TemplateType) -> &'static str {
    SUBJECTS.get(locale)
        .and_then(|subjects| subjects.get(&template_type))
        .unwrap_or_else(|| {
            // Provide default English subject if the specified locale/subject is not found
            SUBJECTS.get("en")
                .and_then(|default_subjects| default_subjects.get(&template_type))
                .unwrap_or(&"")
        })
}

pub fn get_fallback_template(locale: &str, template_type: TemplateType) -> &'static (&'static str, &'static str) {
    TEMPLATES.get(locale)
        .and_then(|templates| templates.get(&template_type))
        .unwrap_or_else(|| {
            // Provide default English template if the specified locale/template is not found
            TEMPLATES.get("en")
                .and_then(|default_templates| default_templates.get(&template_type))
                .unwrap_or(&(
                    include_template!("../../templates/mailer/invite/en.txt.hbs"),
                    include_template!("../../templates/mailer/invite/en.html.hbs"),
                ))
        })
}
