window.onload = function() {
  //<editor-fold desc="Changeable Configuration Block">

  console.log(window.url);
  // the following lines will be replaced by docker/configurator, when it runs in a docker-container
  window.ui = SwaggerUIBundle({
    url: "http://localhost:8383/explorer/swagger.json",
    dom_id: '#swagger-ui',
    deepLinking: true,
    presets: [
      SwaggerUIBundle.presets.apis,
      SwaggerUIStandalonePreset
    ],
    plugins: [
      SwaggerUIBundle.plugins.DownloadUrl
    ],
    layout: "BaseLayout"
  });
  //</editor-fold>
};
