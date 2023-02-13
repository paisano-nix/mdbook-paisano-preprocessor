<div class="std-cell{%- if !readme.is_some() %} std-no-readme{%- endif %}">
<h1 id="{{ "cells-{}"|format(name) }}">
<a class="header" href="{{ "#cells-{}"|format(name) }}">Cell: <code>{{ name }}</code></a>
</h1>

<div class="std-readme-md">
{% if readme.is_some() %}
{{ readme.unwrap()|read_file|offset_headers(1) }}
{%- else -%}
<small><i>No documentation</i></small>
{% endif %}
</div>

</div>

{% for block in blocks %}
{{ block|safe }}
{% endfor %}
