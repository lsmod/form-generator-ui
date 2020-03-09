# Form creators

UI to create forms.
The UI produce a form model to be send to a code generator (React, Angular, React Native)

## Form description:

- Title
- subtitle
- submit label
  Rendering option
- stacked label
- inline

## Field description:

- name
- placeholder
- description/label/title
- type (see Field Type below)
- validation
- required

## Field type:

- Separator
- Separator with text

- Text
- Email
- Url
- Password
- Phone
- LongText
- Date
- Number
- Interval
- Radio
- Checkbox
- SelectList
- Editable SelectList
- Multichoice SelectList
- Multichoice Editable Select List

## Validation

- min length
- max length
- required/optional
- enum list of value

## Bonus

Field that become required when a checkbox got a specific value
