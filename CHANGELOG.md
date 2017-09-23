# Changelog

## v0.7.0 (2017-09-23)

### Fixes

  * Modify `get_parent_and_index` so that an absent parent doesn't make it panic

## v0.6.1 (2017-06-29)

### Fixes

  * Update to `rustler v0.10.1/0.15.1`, which works with OTP 20

## v0.6.0 (2017-06-05)

### Breaking

  * Rename `MeeseeksHtml5ever.parse/1` to `MeeseeksHtml5ever.parse_html/1`

### Enhancements

  * Add `MeeseeksHtml5ever.parse_xml/1`
  * Add `xml5ever` as a dependency
  * Parse CDATA comments as `Meeseeks.Document.Data` nodes
