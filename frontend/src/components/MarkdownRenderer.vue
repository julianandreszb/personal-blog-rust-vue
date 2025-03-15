<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import MarkdownIt from 'markdown-it';
import hljs from 'highlight.js';
import 'highlight.js/styles/obsidian.css';

//TODO: DEMO styles:  https://highlightjs.org/demo
//TODO: MEDIUM GIDE https://medium.com/@hizacharylee/simplify-syntax-highlighting-with-highlight-js-b65af3bdc509

const props = defineProps({
  markdown: {
    type: String,
    required: true,
  },
});

const renderedMarkdown = ref('');
let md = null; // Store the MarkdownIt instance

onMounted(() => {
  try {
    md = new MarkdownIt({
      highlight: (str, lang) => {
        if (lang && hljs.getLanguage(lang)) {
          try {
            return '<pre class="hljs"><code>' +
              hljs.highlight(str, { language: lang, ignoreIllegals: true }).value +
              '</code></pre>'
          } catch (e) {
            console.error("Error highlighting code:", e); // Add this line
          }
        }

        return '<pre class="hljs"><code>' + md.utils.escapeHtml(str) + '</code></pre>'
      },
      html: true,
      linkify: true,
      typographer: true
    })
    
    renderedMarkdown.value = md.render(props.markdown);
  } catch (e) {
    console.error("Error initializing MarkdownIt:", e);
  }
});

onUnmounted(() => {
  md = null; //Clean markdown instance.
});

</script>

<template>
  <div v-html="renderedMarkdown" class="markdown-body"></div>
</template>

<style lang="scss">
.markdown-body {
  font-size: 200%;
}

.markdown-body pre {
  padding: 1rem;
  overflow-x: auto;
  border-radius: 6px;
}
.markdown-body code{
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}

</style>