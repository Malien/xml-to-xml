# XML-to-XML

Yo dawg, I heard you like XML, so I've put XML in yo XML, so you can XML while you XML

### Make your XML glow up ✨

Get your boring XML outa here.
```sh
cat note.xml
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<note>
  <to dearly="true">Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
  <selfclosing />
  <another-one attr="value" />
</note>
```

And make it **323.41%** more enterprize grade by running through `xml-to-xml`

```sh
cat note.xml | xml-to-xml
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<element tag="note">
  <children>
    <element tag="to">
      <attribute name="dearly" value="true"/>
      <children>Tove</children>
    </element>
    <element tag="from">
      <children>Jani</children>
    </element>
    <element tag="heading">
      <children>Reminder</children>
    </element>
    <element tag="body">
      <children>Don't forget me this weekend!</children>
    </element>
    <element tag="selfclosing"/>
    <element tag="another-one">
      <attribute name="attr" value="value"/>
    </element>
  </children>
</element>
```

### Bonus feature
You can stack them together, producing more high-quality XML documents

```sh
cat note.xml | xml-to-xml | xml-to-xml
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<element tag="element">
  <attribute name="tag" value="note"/>
  <children>
    <element tag="children">
      <children>
        <element tag="element">
          <attribute name="tag" value="to"/>
          <children>
            <element tag="attribute">
              <attribute name="name" value="dearly"/>
              <attribute name="value" value="true"/>
            </element>
            <element tag="children">
              <children>Tove</children>
            </element>
          </children>
        </element>
        <element tag="element">
          <attribute name="tag" value="from"/>
          <children>
            <element tag="children">
              <children>Jani</children>
            </element>
          </children>
        </element>
        <element tag="element">
          <attribute name="tag" value="heading"/>
          <children>
            <element tag="children">
              <children>Reminder</children>
            </element>
          </children>
        </element>
        <element tag="element">
          <attribute name="tag" value="body"/>
          <children>
            <element tag="children">
              <children>Don't forget me this weekend!</children>
            </element>
          </children>
        </element>
        <element tag="element">
          <attribute name="tag" value="selfclosing"/>
        </element>
        <element tag="element">
          <attribute name="tag" value="another-one"/>
          <children>
            <element tag="attribute">
              <attribute name="name" value="attr"/>
              <attribute name="value" value="value"/>
            </element>
          </children>
        </element>
      </children>
    </element>
  </children>
</element>
```
