---
layout: page
title: my random musings
tagline: insert tagline here
---
{% include JB/setup %}

Things i've bantered about.

<ul class="posts">
  {% for post in site.posts %}
    <li><span>{{ post.date | date_to_string }}</span> &raquo; <a href="{{ BASE_PATH }}{{ post.url }}">{{ post.title }}</a></li>
  {% endfor %}
</ul>

