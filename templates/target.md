{% if readme.is_some() %}

<div class="std-target">
<h3 id="{{ "targets-{}"|format(name) }}">
<a class="header" href="{{ "#targets-{}"|format(name) }}">Target: <code>{{ name }}</code></a>
</h3>
<small class="std-description"><i>
{% if description.is_some() %}
{{ description.unwrap() }}
{%- else -%}
No description
{% endif %}
</i></small>

<div class="std-readme-md">
{% if readme.is_some() %}
{{ readme.unwrap()|read_file|offset_headers(3) }}
{%- else -%}
<small><i>No documentation</i></small>
{% endif %}
</div>

</div>

{% endif %}
