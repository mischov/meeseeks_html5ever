import Config

config :meeseeks_html5ever, MeeseeksHtml5ever.Native,
  path: "native/meeseeks_html5ever_nif",
  cargo: :system,
  default_features: false,
  features: [],
  mode: :release,
  otp_app: :meeseeks_html5ever,
  crate: :meeseeks_html5ever_nif
