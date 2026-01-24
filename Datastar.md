
# Signals

Each webobject YML can map:
```yaml
state:
  error_message: String = ""
  button_clicked: Boolean = true
```

To:
```html
<div data-signals="{button_clicked: false, error_message: ''}">
  ... component.html
</div>
```

Meaning that we have `global` and `user` objects defined in parent div.
And then for each component the local `state`.


# Common Attributes

### Bind to value
```html
<input data-bind="$foo" />
```

### Bind to text
```html
<input data-text="$foo" />
```

###  Bind to any attribute
```html
<input data-attr:disabled="$foo == ''" />
```

###  Bind to show
```html
<input data-show="$foo == ''" />
```

###  Bind to class
```html
<input data-class:success="$foo" />
```

###  Bind to event
```html
<input data-on:click="$foo == ''" />
```

