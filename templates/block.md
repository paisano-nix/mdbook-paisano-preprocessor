<div class="std-block {%- if !readme.is_some() %}std-no-readme{%- endif %}">
<h2 id="{{ "blocks-{}"|format(name) }}">
<a class="header" href="{{ "#blocks-{}"|format(name) }}">Block: <code>{{ name }}</code></a>
</h2>

<div class="std-readme-md">
{% if readme.is_some() %}
{{ readme.unwrap()|read_file|offset_headers(2) }}
{%- else -%}
<small><i>No documentation</i></small>
{% endif %}
</div>

</div>

{% for target in targets %}
{{ target|safe }}
{% endfor %}
