---
title: nanocld v0.1.0
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
toc_footers: []
includes: []
search: true
highlight_theme: darkula
headingLevel: 2

---

<!-- Generator: Widdershins v4.0.1 -->

<h1 id="nanocld">nanocld v0.1.0</h1>

> Scroll down for code samples, example requests and responses. Select a language for code samples from the tabs above or the mobile navigation menu.

 License: 

<h1 id="nanocld-cargo">cargo</h1>

## list_cargo

<a id="opIdlist_cargo"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /cargoes \
  -H 'Accept: application/json'

```

```http
GET /cargoes HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/cargoes',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/cargoes',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/cargoes', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/cargoes', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/cargoes");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/cargoes", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /cargoes`

*List cargo*

List cargo

<h3 id="list_cargo-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|namespace|query|string|false|Name of the namespace where the cargo are stored|

> Example responses

> 200 Response

```json
[
  {
    "key": "string",
    "namespace_name": "string",
    "image_name": "string",
    "name": "string",
    "binds": [
      "string"
    ]
  }
]
```

<h3 id="list_cargo-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|List of cargo|Inline|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<h3 id="list_cargo-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[CargoItem](#schemacargoitem)]|false|none|[Cargo item is an definition to container create image and start them]|
|» key|string|true|none|none|
|» namespace_name|string|true|none|none|
|» image_name|string|true|none|none|
|» name|string|true|none|none|
|» binds|[string]|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

## create_cargo

<a id="opIdcreate_cargo"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /cargoes \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json'

```

```http
POST /cargoes HTTP/1.1

Content-Type: application/json
Accept: application/json

```

```javascript
const inputBody = '{
  "environnements": [
    "string"
  ],
  "name": "string",
  "proxy_config": {
    "domain_name": "string",
    "target_port": 0,
    "host_ip": "string",
    "template": "string"
  },
  "binds": [
    "string"
  ],
  "image_name": "string"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/cargoes',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Content-Type' => 'application/json',
  'Accept' => 'application/json'
}

result = RestClient.post '/cargoes',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/cargoes', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Content-Type' => 'application/json',
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/cargoes', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/cargoes");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Content-Type": []string{"application/json"},
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/cargoes", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /cargoes`

*Create new cargo*

Create new cargo

> Body parameter

```json
{
  "environnements": [
    "string"
  ],
  "name": "string",
  "proxy_config": {
    "domain_name": "string",
    "target_port": 0,
    "host_ip": "string",
    "template": "string"
  },
  "binds": [
    "string"
  ],
  "image_name": "string"
}
```

<h3 id="create_cargo-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|namespace|query|string|false|Name of the namespace where the cargo will be stored|
|body|body|[CargoPartial](#schemacargopartial)|true|none|

> Example responses

> 201 Response

```json
{
  "key": "string",
  "namespace_name": "string",
  "image_name": "string",
  "name": "string",
  "binds": [
    "string"
  ]
}
```

<h3 id="create_cargo-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|New cargo|[CargoItem](#schemacargoitem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## count_cargo

<a id="opIdcount_cargo"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /cargoes/count \
  -H 'Accept: application/json'

```

```http
GET /cargoes/count HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/cargoes/count',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/cargoes/count',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/cargoes/count', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/cargoes/count', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/cargoes/count");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/cargoes/count", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /cargoes/count`

*Count cargo*

Count cargo

<h3 id="count_cargo-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|namespace|query|string|false|Name of the namespace where the cargo is stored|

> Example responses

> 200 Response

```json
{
  "count": 0
}
```

<h3 id="count_cargo-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Generic delete|[PgGenericCount](#schemapggenericcount)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## delete_cargo_by_name

<a id="opIddelete_cargo_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X DELETE /cargoes/{name} \
  -H 'Accept: application/json'

```

```http
DELETE /cargoes/{name} HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/cargoes/{name}',
{
  method: 'DELETE',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.delete '/cargoes/{name}',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/cargoes/{name}', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('DELETE','/cargoes/{name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/cargoes/{name}");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("DELETE");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("DELETE", "/cargoes/{name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`DELETE /cargoes/{name}`

*Delete cargo by it's name*

Delete cargo by it's name

<h3 id="delete_cargo_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|Name of the cargo|
|namespace|query|string|false|Name of the namespace where the cargo is stored|

> Example responses

> 200 Response

```json
{
  "count": 0
}
```

<h3 id="delete_cargo_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Generic delete|[PgDeleteGeneric](#schemapgdeletegeneric)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="nanocld-cluster">cluster</h1>

## list_cluster

<a id="opIdlist_cluster"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /clusters \
  -H 'Accept: application/json'

```

```http
GET /clusters HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/clusters',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/clusters', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/clusters', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/clusters", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /clusters`

*List all cluster*

List all cluster

<h3 id="list_cluster-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|namespace|query|string|false|Namespace to add cluster in if empty we use 'default' as value|

> Example responses

> 200 Response

```json
{
  "namespace": "string",
  "key": "string",
  "name": "string"
}
```

<h3 id="list_cluster-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|List of cluster for given namespace|[ClusterItem](#schemaclusteritem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|None|

<aside class="success">
This operation does not require authentication
</aside>

## create_cluster

<a id="opIdcreate_cluster"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /clusters \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json'

```

```http
POST /clusters HTTP/1.1

Content-Type: application/json
Accept: application/json

```

```javascript
const inputBody = '{
  "name": "string"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/clusters',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Content-Type' => 'application/json',
  'Accept' => 'application/json'
}

result = RestClient.post '/clusters',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/clusters', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Content-Type' => 'application/json',
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/clusters', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Content-Type": []string{"application/json"},
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/clusters", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /clusters`

*Create new cluster*

Create new cluster

> Body parameter

```json
{
  "name": "string"
}
```

<h3 id="create_cluster-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|namespace|query|string|false|Namespace to add cluster in if empty we use 'default' as value|
|body|body|[ClusterPartial](#schemaclusterpartial)|true|none|

> Example responses

> 201 Response

```json
{
  "namespace": "string",
  "key": "string",
  "name": "string"
}
```

<h3 id="create_cluster-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|Fresh created cluster|[ClusterItem](#schemaclusteritem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|None|

<aside class="success">
This operation does not require authentication
</aside>

## count_cluster

<a id="opIdcount_cluster"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /clusters/count \
  -H 'Accept: application/json'

```

```http
GET /clusters/count HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/count',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/clusters/count',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/clusters/count', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/clusters/count', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/count");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/clusters/count", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /clusters/count`

*Count cluster*

Count cluster

<h3 id="count_cluster-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|namespace|query|string|false|Name of the namespace where the cargo is stored|

> Example responses

> 200 Response

```json
{
  "count": 0
}
```

<h3 id="count_cluster-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Generic delete|[PgGenericCount](#schemapggenericcount)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## delete_cluster_by_name

<a id="opIddelete_cluster_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X DELETE /clusters/{name} \
  -H 'Accept: application/json'

```

```http
DELETE /clusters/{name} HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/{name}',
{
  method: 'DELETE',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.delete '/clusters/{name}',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/clusters/{name}', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('DELETE','/clusters/{name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{name}");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("DELETE");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("DELETE", "/clusters/{name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`DELETE /clusters/{name}`

*Delete cluster by it's name*

Delete cluster by it's name

<h3 id="delete_cluster_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|Name of the cluster|
|namespace|query|string|false|Namespace to add cluster in if empty we use 'default' as value|

> Example responses

> 201 Response

```json
{
  "namespace": "string",
  "key": "string",
  "name": "string"
}
```

<h3 id="delete_cluster_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|Fresh created cluster|[ClusterItem](#schemaclusteritem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## inspect_cluster_by_name

<a id="opIdinspect_cluster_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /clusters/{name}/inspect \
  -H 'Accept: application/json'

```

```http
GET /clusters/{name}/inspect HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/{name}/inspect',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/clusters/{name}/inspect',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/clusters/{name}/inspect', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/clusters/{name}/inspect', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{name}/inspect");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/clusters/{name}/inspect", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /clusters/{name}/inspect`

*Inspect cluster by it's name*

Inspect cluster by it's name

<h3 id="inspect_cluster_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|Name of the cluster|
|namespace|query|string|false|Namespace to add cluster in if empty we use 'default' as value|

> Example responses

> 200 Response

```json
{
  "key": "string",
  "namespace": "string",
  "networks": [
    {
      "docker_network_id": "string",
      "name": "string",
      "namespace": "string",
      "cluster_key": "string",
      "key": "string"
    }
  ],
  "name": "string"
}
```

<h3 id="inspect_cluster_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Cluster information|[ClusterItemWithRelation](#schemaclusteritemwithrelation)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|id name or namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## join_cargo_to_cluster

<a id="opIdjoin_cargo_to_cluster"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /clusters/{name}/join \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json'

```

```http
POST /clusters/{name}/join HTTP/1.1

Content-Type: application/json
Accept: application/json

```

```javascript
const inputBody = '{
  "network": "string",
  "cargo": "string"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/clusters/{name}/join',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Content-Type' => 'application/json',
  'Accept' => 'application/json'
}

result = RestClient.post '/clusters/{name}/join',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/clusters/{name}/join', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Content-Type' => 'application/json',
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/clusters/{name}/join', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{name}/join");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Content-Type": []string{"application/json"},
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/clusters/{name}/join", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /clusters/{name}/join`

*join cargo inside a cluster*

join cargo inside a cluster

> Body parameter

```json
{
  "network": "string",
  "cargo": "string"
}
```

<h3 id="join_cargo_to_cluster-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|Name of the cluster|
|namespace|query|string|false|Namespace to add cluster in if empty we use 'global' as value|
|body|body|[ClusterJoinBody](#schemaclusterjoinbody)|true|none|

> Example responses

> 400 Response

```json
{
  "msg": "string"
}
```

<h3 id="join_cargo_to_cluster-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Cargo joinned successfully|None|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Cluster name of namespace invalid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## start_cluster_by_name

<a id="opIdstart_cluster_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /clusters/{name}/start \
  -H 'Accept: application/json'

```

```http
POST /clusters/{name}/start HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/{name}/start',
{
  method: 'POST',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.post '/clusters/{name}/start',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.post('/clusters/{name}/start', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/clusters/{name}/start', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{name}/start");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/clusters/{name}/start", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /clusters/{name}/start`

*Start all cargo inside cluster*

Start all cargo inside cluster

<h3 id="start_cluster_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|Name of the cluster|
|namespace|query|string|false|Namespace to add cluster in if empty we use 'global' as value|

> Example responses

> 400 Response

```json
{
  "msg": "string"
}
```

<h3 id="start_cluster_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Cargos have been started|None|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Cluster name of namespace invalid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="nanocld-cluster_network">cluster_network</h1>

## list_cluster_network

<a id="opIdlist_cluster_network"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /clusters/{c_name}/networks \
  -H 'Accept: application/json'

```

```http
GET /clusters/{c_name}/networks HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/{c_name}/networks',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/clusters/{c_name}/networks',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/clusters/{c_name}/networks', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/clusters/{c_name}/networks', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{c_name}/networks");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/clusters/{c_name}/networks", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /clusters/{c_name}/networks`

*List network for given cluster*

List network for given cluster

<h3 id="list_cluster_network-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|c_name|path|string|true|name of the cluster|
|namespace|query|string|false|Name of the namespace where the cluster is if empty we use 'default' as value|

> Example responses

> 201 Response

```json
[
  {
    "docker_network_id": "string",
    "name": "string",
    "namespace": "string",
    "cluster_key": "string",
    "key": "string"
  }
]
```

<h3 id="list_cluster_network-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|List of networks|Inline|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<h3 id="list_cluster_network-responseschema">Response Schema</h3>

Status Code **201**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[ClusterNetworkItem](#schemaclusternetworkitem)]|false|none|[Cluster network item]|
|» docker_network_id|string|true|none|none|
|» name|string|true|none|none|
|» namespace|string|true|none|none|
|» cluster_key|string|true|none|none|
|» key|string|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

## create_cluster_network

<a id="opIdcreate_cluster_network"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /clusters/{c_name}/networks \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json'

```

```http
POST /clusters/{c_name}/networks HTTP/1.1

Content-Type: application/json
Accept: application/json

```

```javascript
const inputBody = '{
  "name": "string"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/clusters/{c_name}/networks',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Content-Type' => 'application/json',
  'Accept' => 'application/json'
}

result = RestClient.post '/clusters/{c_name}/networks',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/clusters/{c_name}/networks', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Content-Type' => 'application/json',
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/clusters/{c_name}/networks', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{c_name}/networks");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Content-Type": []string{"application/json"},
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/clusters/{c_name}/networks", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /clusters/{c_name}/networks`

*Create a network for given cluster*

Create a network for given cluster

> Body parameter

```json
{
  "name": "string"
}
```

<h3 id="create_cluster_network-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|c_name|path|string|true|name of the cluster|
|namespace|query|string|false|Name of the namespace where the cluster is if empty we use 'default' as value|
|body|body|[ClusterNetworkPartial](#schemaclusternetworkpartial)|true|none|

> Example responses

> 201 Response

```json
{
  "docker_network_id": "string",
  "name": "string",
  "namespace": "string",
  "cluster_key": "string",
  "key": "string"
}
```

<h3 id="create_cluster_network-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|List of networks|[ClusterNetworkItem](#schemaclusternetworkitem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## delete_cluster_network_by_name

<a id="opIddelete_cluster_network_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X DELETE /clusters/{c_name}/networks/{n_name} \
  -H 'Accept: application/json'

```

```http
DELETE /clusters/{c_name}/networks/{n_name} HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/{c_name}/networks/{n_name}',
{
  method: 'DELETE',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.delete '/clusters/{c_name}/networks/{n_name}',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/clusters/{c_name}/networks/{n_name}', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('DELETE','/clusters/{c_name}/networks/{n_name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{c_name}/networks/{n_name}");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("DELETE");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("DELETE", "/clusters/{c_name}/networks/{n_name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`DELETE /clusters/{c_name}/networks/{n_name}`

*Delete network by it's name for given cluster in given namespace*

Delete network by it's name for given cluster in given namespace

<h3 id="delete_cluster_network_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|c_name|path|string|true|name of the cluster|
|n_name|path|string|true|name of the network|
|namespace|query|string|false|Name of the namespace where the cluster is if empty we use 'default' as value|

> Example responses

> 200 Response

```json
{
  "count": 0
}
```

<h3 id="delete_cluster_network_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Pg delete response|[PgDeleteGeneric](#schemapgdeletegeneric)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Cluster network not found|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## inspect_cluster_network_by_name

<a id="opIdinspect_cluster_network_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /clusters/{c_name}/networks/{n_name}/inspect \
  -H 'Accept: application/json'

```

```http
GET /clusters/{c_name}/networks/{n_name}/inspect HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/{c_name}/networks/{n_name}/inspect',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/clusters/{c_name}/networks/{n_name}/inspect',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/clusters/{c_name}/networks/{n_name}/inspect', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/clusters/{c_name}/networks/{n_name}/inspect', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{c_name}/networks/{n_name}/inspect");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/clusters/{c_name}/networks/{n_name}/inspect", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /clusters/{c_name}/networks/{n_name}/inspect`

*Inspect network by it's name for given cluster in given namespace*

Inspect network by it's name for given cluster in given namespace

<h3 id="inspect_cluster_network_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|c_name|path|string|true|name of the cluster|
|n_name|path|string|true|name of the network|
|namespace|query|string|false|Name of the namespace where the cluster is if empty we use 'default' as value|

> Example responses

> 200 Response

```json
{
  "docker_network_id": "string",
  "name": "string",
  "namespace": "string",
  "cluster_key": "string",
  "key": "string"
}
```

<h3 id="inspect_cluster_network_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Network item|[ClusterNetworkItem](#schemaclusternetworkitem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## count_cluster_network_by_namespace

<a id="opIdcount_cluster_network_by_namespace"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /networks/count \
  -H 'Accept: application/json'

```

```http
GET /networks/count HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/networks/count',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/networks/count',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/networks/count', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/networks/count', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/networks/count");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/networks/count", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /networks/count`

*Count cluster*

Count cluster

<h3 id="count_cluster_network_by_namespace-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|namespace|query|string|false|Name of the namespace where the cargo is stored|

> Example responses

> 200 Response

```json
{
  "count": 0
}
```

<h3 id="count_cluster_network_by_namespace-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Generic delete|[PgGenericCount](#schemapggenericcount)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="nanocld-cluster_variable">cluster_variable</h1>

## list_cluster_variable

<a id="opIdlist_cluster_variable"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /clusters/{c_name}/variables \
  -H 'Accept: application/json'

```

```http
GET /clusters/{c_name}/variables HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/{c_name}/variables',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/clusters/{c_name}/variables',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/clusters/{c_name}/variables', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/clusters/{c_name}/variables', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{c_name}/variables");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/clusters/{c_name}/variables", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /clusters/{c_name}/variables`

*List variable of a cluster*

List variable of a cluster

<h3 id="list_cluster_variable-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|c_name|path|string|true|name of the cluster|
|namespace|query|string|false|Name of the namespace where the cluster is stored is empty we use 'global' as value|

> Example responses

> 200 Response

```json
[
  {
    "value": "string",
    "key": "string",
    "cluster_key": "string",
    "name": "string"
  }
]
```

<h3 id="list_cluster_variable-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|List of variable for given cluster|Inline|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Cluster name or Namespace not valid|[ApiError](#schemaapierror)|

<h3 id="list_cluster_variable-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[ClusterVariableItem](#schemaclustervariableitem)]|false|none|none|
|» value|string|true|none|none|
|» key|string|true|none|none|
|» cluster_key|string|true|none|none|
|» name|string|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

## create_cluster_variable

<a id="opIdcreate_cluster_variable"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /clusters/{c_name}/variables \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json'

```

```http
POST /clusters/{c_name}/variables HTTP/1.1

Content-Type: application/json
Accept: application/json

```

```javascript
const inputBody = '{
  "name": "string",
  "value": "string"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/clusters/{c_name}/variables',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Content-Type' => 'application/json',
  'Accept' => 'application/json'
}

result = RestClient.post '/clusters/{c_name}/variables',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/clusters/{c_name}/variables', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Content-Type' => 'application/json',
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/clusters/{c_name}/variables', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{c_name}/variables");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Content-Type": []string{"application/json"},
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/clusters/{c_name}/variables", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /clusters/{c_name}/variables`

*Create cluster variable*

Create cluster variable

> Body parameter

```json
{
  "name": "string",
  "value": "string"
}
```

<h3 id="create_cluster_variable-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|c_name|path|string|true|name of the cluster|
|namespace|query|string|false|Name of the namespace where the cluster is stored is empty we use 'global' as value|
|body|body|[ClusterVariablePartial](#schemaclustervariablepartial)|true|none|

> Example responses

> 200 Response

```json
{
  "value": "string",
  "key": "string",
  "cluster_key": "string",
  "name": "string"
}
```

<h3 id="create_cluster_variable-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Fresh cluster variable|[ClusterVariableItem](#schemaclustervariableitem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Cluster name or Namespace not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## delete_cluster_variable

<a id="opIddelete_cluster_variable"></a>

> Code samples

```shell
# You can also use wget
curl -X DELETE /clusters/{c_name}/variables/{v_name} \
  -H 'Accept: application/json'

```

```http
DELETE /clusters/{c_name}/variables/{v_name} HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/clusters/{c_name}/variables/{v_name}',
{
  method: 'DELETE',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.delete '/clusters/{c_name}/variables/{v_name}',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/clusters/{c_name}/variables/{v_name}', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('DELETE','/clusters/{c_name}/variables/{v_name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/clusters/{c_name}/variables/{v_name}");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("DELETE");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("DELETE", "/clusters/{c_name}/variables/{v_name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`DELETE /clusters/{c_name}/variables/{v_name}`

*Delete cluster variable by it's name*

Delete cluster variable by it's name

<h3 id="delete_cluster_variable-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|c_name|path|string|true|name of the cluster|
|v_name|path|string|true|name of the variable|
|namespace|query|string|false|Name of the namespace where the cluster is stored is empty we use 'global' as value|

> Example responses

> 200 Response

```json
{
  "count": 0
}
```

<h3 id="delete_cluster_variable-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Generic delete response|[PgDeleteGeneric](#schemapgdeletegeneric)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|[ApiError](#schemaapierror)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Cluster name or Namespace not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="nanocld-git_repository">git_repository</h1>

## list_git_repository

<a id="opIdlist_git_repository"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /git_repositories \
  -H 'Accept: application/json'

```

```http
GET /git_repositories HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/git_repositories',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/git_repositories',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/git_repositories', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/git_repositories', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/git_repositories");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/git_repositories", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /git_repositories`

*List all git repository*

List all git repository

> Example responses

> 200 Response

```json
[
  {
    "url": "string",
    "name": "string",
    "default_branch": "string",
    "source": "github"
  }
]
```

<h3 id="list_git_repository-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Array of git_repository|Inline|

<h3 id="list_git_repository-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[GitRepositoryItem](#schemagitrepositoryitem)]|false|none|[Git repository are used to have project definition to deploy cargo]|
|» url|string|true|none|none|
|» name|string|true|none|none|
|» default_branch|string|true|none|none|
|» source|[GitRepositorySourceType](#schemagitrepositorysourcetype)|true|none|Git repository source types|

#### Enumerated Values

|Property|Value|
|---|---|
|source|github|
|source|gitlab|
|source|local|

<aside class="success">
This operation does not require authentication
</aside>

## create_git_repository

<a id="opIdcreate_git_repository"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /git_repositories \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json'

```

```http
POST /git_repositories HTTP/1.1

Content-Type: application/json
Accept: application/json

```

```javascript
const inputBody = '{
  "name": "string",
  "url": "string"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/git_repositories',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Content-Type' => 'application/json',
  'Accept' => 'application/json'
}

result = RestClient.post '/git_repositories',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/git_repositories', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Content-Type' => 'application/json',
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/git_repositories', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/git_repositories");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Content-Type": []string{"application/json"},
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/git_repositories", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /git_repositories`

*Create new git repository*

Create new git repository

> Body parameter

```json
{
  "name": "string",
  "url": "string"
}
```

<h3 id="create_git_repository-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|body|body|[GitRepositoryPartial](#schemagitrepositorypartial)|true|none|

> Example responses

> 201 Response

```json
{
  "url": "string",
  "name": "string",
  "default_branch": "string",
  "source": "github"
}
```

<h3 id="create_git_repository-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|Fresh created git_repository|[GitRepositoryItem](#schemagitrepositoryitem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|None|
|422|[Unprocessable Entity](https://tools.ietf.org/html/rfc2518#section-10.3)|The provided payload is not valid|None|

<aside class="success">
This operation does not require authentication
</aside>

## delete_git_repository_by_name

<a id="opIddelete_git_repository_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X DELETE /git_repositories/{name} \
  -H 'Accept: application/json'

```

```http
DELETE /git_repositories/{name} HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/git_repositories/{name}',
{
  method: 'DELETE',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.delete '/git_repositories/{name}',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/git_repositories/{name}', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('DELETE','/git_repositories/{name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/git_repositories/{name}");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("DELETE");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("DELETE", "/git_repositories/{name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`DELETE /git_repositories/{name}`

*Delete git repository by it's name*

Delete git repository by it's name

<h3 id="delete_git_repository_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|id|path|string|true|Name of git repository|

> Example responses

> 201 Response

```json
{
  "count": 0
}
```

<h3 id="delete_git_repository_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|Number of entry deleted|[PgDeleteGeneric](#schemapgdeletegeneric)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name not valid|None|

<aside class="success">
This operation does not require authentication
</aside>

## build_git_repository_by_name

<a id="opIdbuild_git_repository_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /git_repositories/{name}/build \
  -H 'Accept: nanocl/streaming-v1'

```

```http
POST /git_repositories/{name}/build HTTP/1.1

Accept: nanocl/streaming-v1

```

```javascript

const headers = {
  'Accept':'nanocl/streaming-v1'
};

fetch('/git_repositories/{name}/build',
{
  method: 'POST',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'nanocl/streaming-v1'
}

result = RestClient.post '/git_repositories/{name}/build',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'nanocl/streaming-v1'
}

r = requests.post('/git_repositories/{name}/build', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'nanocl/streaming-v1',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/git_repositories/{name}/build', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/git_repositories/{name}/build");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"nanocl/streaming-v1"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/git_repositories/{name}/build", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /git_repositories/{name}/build`

*Build docker image for given git repository name*

Build docker image for given git repository name

<h3 id="build_git_repository_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|Name of git repository|

> Example responses

> 200 Response

<h3 id="build_git_repository_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Stream of building process|string|
|304|[Not Modified](https://tools.ietf.org/html/rfc7232#section-4.1)|Content is up to date with selected branch|None|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|Generic database error|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace name or git repository name not valid|None|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="nanocld-namespace">namespace</h1>

## list_namespace

<a id="opIdlist_namespace"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /namespaces \
  -H 'Accept: application/json'

```

```http
GET /namespaces HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/namespaces',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/namespaces',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/namespaces', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/namespaces', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/namespaces");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/namespaces", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /namespaces`

*List all namespace*

List all namespace

> Example responses

> 200 Response

```json
[
  {
    "name": "string"
  }
]
```

<h3 id="list_namespace-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Array of namespace|Inline|

<h3 id="list_namespace-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[NamespaceItem](#schemanamespaceitem)]|false|none|[Namespace to encapsulate clusters]|
|» name|string|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

## create_namespace

<a id="opIdcreate_namespace"></a>

> Code samples

```shell
# You can also use wget
curl -X POST /namespaces \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json'

```

```http
POST /namespaces HTTP/1.1

Content-Type: application/json
Accept: application/json

```

```javascript
const inputBody = '{
  "name": "string"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/namespaces',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Content-Type' => 'application/json',
  'Accept' => 'application/json'
}

result = RestClient.post '/namespaces',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/namespaces', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Content-Type' => 'application/json',
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('POST','/namespaces', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/namespaces");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("POST");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Content-Type": []string{"application/json"},
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "/namespaces", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`POST /namespaces`

*Create new namespace*

Create new namespace

> Body parameter

```json
{
  "name": "string"
}
```

<h3 id="create_namespace-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|body|body|[NamespacePartial](#schemanamespacepartial)|true|none|

> Example responses

> 201 Response

```json
{
  "name": "string"
}
```

<h3 id="create_namespace-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|fresh created namespace|[NamespaceItem](#schemanamespaceitem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|generic database error|[ApiError](#schemaapierror)|
|422|[Unprocessable Entity](https://tools.ietf.org/html/rfc2518#section-10.3)|the provided payload is not valid|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

## delete_namespace_by_name

<a id="opIddelete_namespace_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X DELETE /namespaces/{name} \
  -H 'Accept: application/json'

```

```http
DELETE /namespaces/{name} HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/namespaces/{name}',
{
  method: 'DELETE',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.delete '/namespaces/{name}',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/namespaces/{name}', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('DELETE','/namespaces/{name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/namespaces/{name}");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("DELETE");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("DELETE", "/namespaces/{name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`DELETE /namespaces/{name}`

*Delete namespace by it's name*

Delete namespace by it's name

<h3 id="delete_namespace_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|name of the namespace|

> Example responses

> 200 Response

```json
{
  "count": 0
}
```

<h3 id="delete_namespace_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|database generic delete response|[PgDeleteGeneric](#schemapgdeletegeneric)|

<aside class="success">
This operation does not require authentication
</aside>

## inspect_namespace_by_name

<a id="opIdinspect_namespace_by_name"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /namespaces/{name}/inspect \
  -H 'Accept: application/json'

```

```http
GET /namespaces/{name}/inspect HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/namespaces/{name}/inspect',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/namespaces/{name}/inspect',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/namespaces/{name}/inspect', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/namespaces/{name}/inspect', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/namespaces/{name}/inspect");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/namespaces/{name}/inspect", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /namespaces/{name}/inspect`

*Inspect namespace by it's name*

Inspect namespace by it's name

<h3 id="inspect_namespace_by_name-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|name of the namespace|

> Example responses

> 200 Response

```json
{
  "name": "string"
}
```

<h3 id="inspect_namespace_by_name-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Namespace found|[NamespaceItem](#schemanamespaceitem)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|Namespace not found|[ApiError](#schemaapierror)|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="nanocld-nginx_template">nginx_template</h1>

## list_nginx_template

<a id="opIdlist_nginx_template"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /nginx_templates \
  -H 'Accept: application/json'

```

```http
GET /nginx_templates HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/nginx_templates',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/nginx_templates',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/nginx_templates', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/nginx_templates', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/nginx_templates");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/nginx_templates", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`GET /nginx_templates`

*List all nginx template*

List all nginx template

> Example responses

> 200 Response

```json
[
  {
    "name": "string",
    "content": "string"
  }
]
```

<h3 id="list_nginx_template-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Array of nginx templates|Inline|

<h3 id="list_nginx_template-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[NginxTemplateItem](#schemanginxtemplateitem)]|false|none|none|
|» name|string|true|none|none|
|» content|string|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

# Schemas

<h2 id="tocS_PgDeleteGeneric">PgDeleteGeneric</h2>
<!-- backwards compatibility -->
<a id="schemapgdeletegeneric"></a>
<a id="schema_PgDeleteGeneric"></a>
<a id="tocSpgdeletegeneric"></a>
<a id="tocspgdeletegeneric"></a>

```json
{
  "count": 0
}

```

Generic postgresql delete response

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|count|integer|true|none|none|

<h2 id="tocS_ApiError">ApiError</h2>
<!-- backwards compatibility -->
<a id="schemaapierror"></a>
<a id="schema_ApiError"></a>
<a id="tocSapierror"></a>
<a id="tocsapierror"></a>

```json
{
  "msg": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|msg|string|true|none|none|

<h2 id="tocS_CargoProxyConfigItem">CargoProxyConfigItem</h2>
<!-- backwards compatibility -->
<a id="schemacargoproxyconfigitem"></a>
<a id="schema_CargoProxyConfigItem"></a>
<a id="tocScargoproxyconfigitem"></a>
<a id="tocscargoproxyconfigitem"></a>

```json
{
  "cargo_key": "string",
  "host_ip": "string",
  "target_port": 0,
  "template": "string",
  "domain_name": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|cargo_key|string|true|none|none|
|host_ip|string|true|none|none|
|target_port|integer(int32)|true|none|none|
|template|string|true|none|none|
|domain_name|string|true|none|none|

<h2 id="tocS_ClusterItem">ClusterItem</h2>
<!-- backwards compatibility -->
<a id="schemaclusteritem"></a>
<a id="schema_ClusterItem"></a>
<a id="tocSclusteritem"></a>
<a id="tocsclusteritem"></a>

```json
{
  "namespace": "string",
  "key": "string",
  "name": "string"
}

```

Cluster used to encapsulate networks

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|namespace|string|true|none|none|
|key|string|true|none|none|
|name|string|true|none|none|

<h2 id="tocS_GitRepositorySourceType">GitRepositorySourceType</h2>
<!-- backwards compatibility -->
<a id="schemagitrepositorysourcetype"></a>
<a id="schema_GitRepositorySourceType"></a>
<a id="tocSgitrepositorysourcetype"></a>
<a id="tocsgitrepositorysourcetype"></a>

```json
"github"

```

Git repository source types

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|string|false|none|Git repository source types|

#### Enumerated Values

|Property|Value|
|---|---|
|*anonymous*|github|
|*anonymous*|gitlab|
|*anonymous*|local|

<h2 id="tocS_PgGenericCount">PgGenericCount</h2>
<!-- backwards compatibility -->
<a id="schemapggenericcount"></a>
<a id="schema_PgGenericCount"></a>
<a id="tocSpggenericcount"></a>
<a id="tocspggenericcount"></a>

```json
{
  "count": 0
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|count|integer(int64)|true|none|none|

<h2 id="tocS_CargoPartial">CargoPartial</h2>
<!-- backwards compatibility -->
<a id="schemacargopartial"></a>
<a id="schema_CargoPartial"></a>
<a id="tocScargopartial"></a>
<a id="tocscargopartial"></a>

```json
{
  "environnements": [
    "string"
  ],
  "name": "string",
  "proxy_config": {
    "domain_name": "string",
    "target_port": 0,
    "host_ip": "string",
    "template": "string"
  },
  "binds": [
    "string"
  ],
  "image_name": "string"
}

```

Cargo partial

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|environnements|[string]|false|none|none|
|name|string|true|none|none|
|proxy_config|[CargoProxyConfigPartial](#schemacargoproxyconfigpartial)|false|none|none|
|binds|[string]|false|none|none|
|image_name|string|true|none|none|

<h2 id="tocS_ClusterVariablePartial">ClusterVariablePartial</h2>
<!-- backwards compatibility -->
<a id="schemaclustervariablepartial"></a>
<a id="schema_ClusterVariablePartial"></a>
<a id="tocSclustervariablepartial"></a>
<a id="tocsclustervariablepartial"></a>

```json
{
  "name": "string",
  "value": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|none|
|value|string|true|none|none|

<h2 id="tocS_CargoProxyConfigPartial">CargoProxyConfigPartial</h2>
<!-- backwards compatibility -->
<a id="schemacargoproxyconfigpartial"></a>
<a id="schema_CargoProxyConfigPartial"></a>
<a id="tocScargoproxyconfigpartial"></a>
<a id="tocscargoproxyconfigpartial"></a>

```json
{
  "domain_name": "string",
  "target_port": 0,
  "host_ip": "string",
  "template": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|domain_name|string|true|none|none|
|target_port|integer(int32)|true|none|none|
|host_ip|string|true|none|none|
|template|string|true|none|none|

<h2 id="tocS_NginxTemplateItem">NginxTemplateItem</h2>
<!-- backwards compatibility -->
<a id="schemanginxtemplateitem"></a>
<a id="schema_NginxTemplateItem"></a>
<a id="tocSnginxtemplateitem"></a>
<a id="tocsnginxtemplateitem"></a>

```json
{
  "name": "string",
  "content": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|none|
|content|string|true|none|none|

<h2 id="tocS_GitRepositoryPartial">GitRepositoryPartial</h2>
<!-- backwards compatibility -->
<a id="schemagitrepositorypartial"></a>
<a id="schema_GitRepositoryPartial"></a>
<a id="tocSgitrepositorypartial"></a>
<a id="tocsgitrepositorypartial"></a>

```json
{
  "name": "string",
  "url": "string"
}

```

Partial Git repository

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|none|
|url|string|true|none|none|

<h2 id="tocS_NamespaceItem">NamespaceItem</h2>
<!-- backwards compatibility -->
<a id="schemanamespaceitem"></a>
<a id="schema_NamespaceItem"></a>
<a id="tocSnamespaceitem"></a>
<a id="tocsnamespaceitem"></a>

```json
{
  "name": "string"
}

```

Namespace to encapsulate clusters

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|none|

<h2 id="tocS_ClusterJoinBody">ClusterJoinBody</h2>
<!-- backwards compatibility -->
<a id="schemaclusterjoinbody"></a>
<a id="schema_ClusterJoinBody"></a>
<a id="tocSclusterjoinbody"></a>
<a id="tocsclusterjoinbody"></a>

```json
{
  "network": "string",
  "cargo": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|network|string|true|none|none|
|cargo|string|true|none|none|

<h2 id="tocS_ClusterPartial">ClusterPartial</h2>
<!-- backwards compatibility -->
<a id="schemaclusterpartial"></a>
<a id="schema_ClusterPartial"></a>
<a id="tocSclusterpartial"></a>
<a id="tocsclusterpartial"></a>

```json
{
  "name": "string"
}

```

Partial cluster

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|none|

<h2 id="tocS_NamespacePartial">NamespacePartial</h2>
<!-- backwards compatibility -->
<a id="schemanamespacepartial"></a>
<a id="schema_NamespacePartial"></a>
<a id="tocSnamespacepartial"></a>
<a id="tocsnamespacepartial"></a>

```json
{
  "name": "string"
}

```

Partial namespace

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|none|

<h2 id="tocS_ClusterVariableItem">ClusterVariableItem</h2>
<!-- backwards compatibility -->
<a id="schemaclustervariableitem"></a>
<a id="schema_ClusterVariableItem"></a>
<a id="tocSclustervariableitem"></a>
<a id="tocsclustervariableitem"></a>

```json
{
  "value": "string",
  "key": "string",
  "cluster_key": "string",
  "name": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|value|string|true|none|none|
|key|string|true|none|none|
|cluster_key|string|true|none|none|
|name|string|true|none|none|

<h2 id="tocS_ClusterNetworkItem">ClusterNetworkItem</h2>
<!-- backwards compatibility -->
<a id="schemaclusternetworkitem"></a>
<a id="schema_ClusterNetworkItem"></a>
<a id="tocSclusternetworkitem"></a>
<a id="tocsclusternetworkitem"></a>

```json
{
  "docker_network_id": "string",
  "name": "string",
  "namespace": "string",
  "cluster_key": "string",
  "key": "string"
}

```

Cluster network item

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|docker_network_id|string|true|none|none|
|name|string|true|none|none|
|namespace|string|true|none|none|
|cluster_key|string|true|none|none|
|key|string|true|none|none|

<h2 id="tocS_GitRepositoryItem">GitRepositoryItem</h2>
<!-- backwards compatibility -->
<a id="schemagitrepositoryitem"></a>
<a id="schema_GitRepositoryItem"></a>
<a id="tocSgitrepositoryitem"></a>
<a id="tocsgitrepositoryitem"></a>

```json
{
  "url": "string",
  "name": "string",
  "default_branch": "string",
  "source": "github"
}

```

Git repository are used to have project definition to deploy cargo

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|url|string|true|none|none|
|name|string|true|none|none|
|default_branch|string|true|none|none|
|source|[GitRepositorySourceType](#schemagitrepositorysourcetype)|true|none|Git repository source types|

<h2 id="tocS_ClusterNetworkPartial">ClusterNetworkPartial</h2>
<!-- backwards compatibility -->
<a id="schemaclusternetworkpartial"></a>
<a id="schema_ClusterNetworkPartial"></a>
<a id="tocSclusternetworkpartial"></a>
<a id="tocsclusternetworkpartial"></a>

```json
{
  "name": "string"
}

```

Cluster network partial

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|none|

<h2 id="tocS_ClusterItemWithRelation">ClusterItemWithRelation</h2>
<!-- backwards compatibility -->
<a id="schemaclusteritemwithrelation"></a>
<a id="schema_ClusterItemWithRelation"></a>
<a id="tocSclusteritemwithrelation"></a>
<a id="tocsclusteritemwithrelation"></a>

```json
{
  "key": "string",
  "namespace": "string",
  "networks": [
    {
      "docker_network_id": "string",
      "name": "string",
      "namespace": "string",
      "cluster_key": "string",
      "key": "string"
    }
  ],
  "name": "string"
}

```

Cluster item with his relations

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|key|string|true|none|none|
|namespace|string|true|none|none|
|networks|[[ClusterNetworkItem](#schemaclusternetworkitem)]|false|none|[Cluster network item]|
|name|string|true|none|none|

<h2 id="tocS_CargoItem">CargoItem</h2>
<!-- backwards compatibility -->
<a id="schemacargoitem"></a>
<a id="schema_CargoItem"></a>
<a id="tocScargoitem"></a>
<a id="tocscargoitem"></a>

```json
{
  "key": "string",
  "namespace_name": "string",
  "image_name": "string",
  "name": "string",
  "binds": [
    "string"
  ]
}

```

Cargo item is an definition to container create image and start them

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|key|string|true|none|none|
|namespace_name|string|true|none|none|
|image_name|string|true|none|none|
|name|string|true|none|none|
|binds|[string]|true|none|none|

