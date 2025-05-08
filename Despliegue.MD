# Tutorial
En este tutorial:
- Desplegaremos un nodo bootstrap (Nodo1)
- Desplegaremos un nodo addressable (Nodo2)
- Conectaremos el nodo addressable con el bootstrap
- Crearemos una gobernanza donde el Nodo1 será el dueño de la gobernanza, habrá un `schema` con el contrato del ejemplo, el Nodo1 será `issuer` y `creator` para ese schema y el Nodo2 será `witness`.

## Nota
En cada título se indica a qué nodo hay que realizar las peticiones. Igualmente los nodos están escuchando en puertos diferentes por lo que no debería haber duda.

## Terminología
- Nodo bootstrap: Los nodos bootstrap son un tipo especial de nodo cuya función es otorgar acceso a la red al resto de los nodos. Es decir, cualquier nodo que desee unirse a nuestra red debe conectarse a uno de estos, por lo que deben ser accesibles desde otros nodos; su dirección se indica en la configuración del nodo.
Lo habitual es que una red cuente con varios nodos bootstrap para garantizar la resiliencia de las conexiones. Si un nodo tiene configurados uno o más nodos bootstrap y ninguno está disponible (por problemas de red, cambios de IP, etc.), quedará inoperativo, ya que al no poder conectarse a la red no tiene sentido que permanezca activo.
El único nodo que no necesita conectarse a otro nodo bootstrap es el primer nodo bootstrap de la red. A nivel de KoreLedger, este tipo de nodo puede desempeñar cualquier rol (los roles se explicarán más adelante). Este nodo utiliza el protocolo de comunicación Tell.

- Nodo addressable: nodo accesible a través de Internet. Básicamente, es similar a un nodo bootstrap, pero no proporciona acceso a la red. Puede desempeñar cualquiera de los roles y también utiliza el protocolo Tell.

- Nodo efímero: nodo que, en principio, no es accesible a través de Internet. Puede desempeñar cualquiera de los roles y utiliza el protocolo Request–Response. Está pensado principalmente para usuarios comunes, ya que puede ejecutarse en un móvil, en un ordenador de oficina o en cualquier otro dispositivo que no sea accesible a través de internet.

- Enfoques futuros respecto a los nodos: en el futuro, el nodo bootstrap probablemente se limite únicamente a su función de dar acceso a la red y no desempeñe ningún rol dentro de KoreLedger. De este modo, estos nodos contarían solo con lo estrictamente indispensable y serían muy ligeros. Esto es posible porque, si necesitamos asumir roles dentro de KoreLedger, podemos recurrir a los nodos addressables.

- Planteamiento de una red:
Una red se puede diseñar de muchas formas, sobre todo en entornos locales donde todos los nodos se ven entre sí. Por ejemplo, podríamos usar únicamente nodos bootstrap o combinar un único bootstrap con el resto de nodos efímeros.
Sin embargo, el esquema ideal para una red en producción es:
    * Nodos bootstrap para gestionar las conexiones iniciales.
    * Nodos addressables para asumir los distintos roles dentro de KoreLedger.
    * Nodos efímeros para los clientes finales.

- Protocolo Tell: Protocolo desarrollado por KoreLedger para sus nodos dentro del marco del LibP2P, este protocolo funciona como un Request-Response pero una vez enviado el mensaje cierra el socket, sin esperar a la respuesta, este comportamiento es asíncrono puro.

- Gobernanza: La gobernanza es la base de KoreLedger, basicamente es una estructura que establece las normas, quién hace qué cosa y qué cosas se pueden hacer. Las gobernanzas tienen members, roles, schemas y policies.

- Members: son los miembros que forman una gobernanza, un nodo tiene que ser miembro de una gobernanza para poder recibir la copia de esta y ejercer cualquier rol.

- Sujetos de Trazabilidad: Los sujetos de trazabilidad son los sujetos creados en base a un schema definido en una gobernanza, aunque objetivamente una gobernanza también es un sujeto de trazabilidad, cuando hablemos de sujetos de trazabilidad nos referiremos a los creados a partir del schema de una gobernanza y no a las gobernanzas para que nos sea más fácil diferenciarlos. La relación gobernanza - sujetos de trazabilidad es una relación de uno a muchos, ya que una gobernanza puede tener "infinitos" sujetos de trazabilidad pero un sujeto de trazabilidad solo puede pertenecer a una gobernanza.

- Roles: como su nombre indica son los roles dentro de la gobernanza, un rol se puede aplicar a un schema o a la propia gobernanza:
    * Approver: rol de aprobador, los eventos de Fact de la gobernanza requieren de aprobación estos nodos indicarán si están de acuerdo con los cambios del evento de Fact o no, si no se cumple el quorum, este evento de Fact quedará registrado pero no se aplicará. Este rol solo existe para las gobernanzas.
    * Evaluator: Se encarga de evaluar lo eventos de Fact. Para los sujetos de trazabilidad ejecutará el contrato y para las gobernanzas realiza comprobaciones (como por ejemplo si se añade un nuevo rol a un nodo comprobar que ese nodo sea miembro), este rol se encarga de decir si es correcto el evento enviado o no.
    * Validator: Se encarga de validar que la secuencia de los eventos sea la correcta y no se produzcan Forks en el ledger.
    * Witness: Se encarga de recibir las copias de los ledgers de los sujetos de trazabilidad, proporcionan reciliencia a la red ya que permite que un nodo recupere su ledger en caso de que lo haya perdido.
    * Issuer: Se encarga de firmar los eventos.
    * Creator: Permite crear sujetos de trazabilidad a un miembro, este rol solo está disponible para los schemas.

- Schema: Un schema permite crear sujetos de trazabilidad, se proporciona un Raw (el contrato en base64, el contrato de este ejemplo se encuentra en src/lib.rs) y un estado inicial para el sujeto de trazabilidad.
- Policies: son las políticas del quorum que se tiene que cumplir para que un protocolo sea válido. Los protocolos que tienen quorum son la evaluación, la validación y la aprobación (solo para gobernanzas), hay 3 tipos de quorum:
    * Majority: La mayoría tiene que dar el visto bueno.
    * Fixed(X): donde X es la cantidad de nodos que tienen que dar el visto bueno.
    * Percentage(X): donde X es un número del 1 al 100 el cual indica el porcentaje de nodos que tiene que dar el visto bueno.

## Crear Red para los nodos y los clientes:
Creamos una red para que los nodos se comuniquen entre ellos.
```bash
docker network create --driver bridge --subnet 172.28.1.0/24 kore-example-network
```

## Desplegar el NODO1:
- RUST_LOG: Nivel de logs del nodo.
- KORE_PASSWORD: Contraseña para el material criptográfico que genera el nodo, el material es un pkcs8 cifrado con pkcs5, el material criptográfico hace que el nodo sea único en la red, si se pierde este material se pierde ese nodo para siempre, por eso es muy importante respaldar este material.
- KORE_NETWORK_NODE_TYPE: Variable para indicar el tipo de nodo, si no se establece la variable el nodo será por defecto bootstrap.
- KORE_NETWORK_LISTEN_ADDRESSES: variable para indicar la donde va a escuchar el nodo a otros nodos de la red.
- KORE_HTTPS_DOC: Variable para activar la documentación, al acceder a http://localhost:3000/doc desde un navegador podrá utilizar la API del nodo con una interfaz gráfica y acceder al JSON del OpenApi desde http://localhost:3000/doc/koreapi.json

Se ha mapeado los puertos 3000 para atacar la API del nodo, el 50000 para comunicaciones entre nodos y el 3050 para prometheus, además se han mapeado los volumenes de las bases de datos y el material criptográfico.
```bash
docker run --name kore_example1 -p 3000:3000 -p 50000:50000 -p 3050:3050 \
-e RUST_LOG=info \
-e KORE_HTTPS_DOC: true \
-e KORE_PASSWORD=koreledger \
-e KORE_NETWORK_NODE_TYPE=Bootstrap \
-e KORE_NETWORK_LISTEN_ADDRESSES=/ip4/0.0.0.0/tcp/50000 \
-v ./kore_example1_db:/db \
-v ./kore_example1_keys:/keys \
--network kore-example-network \
--ip 172.28.1.100 \
koreledgerhub/kore-http:0.7.0-rockdb-prometheus
```

## Nodo1 - Optener peer-id del nodo1:
Vamos a obtener el peer-id del nodo que acabamos de desplegar, necesitamos ese peer-id para que el otro nodo que desplegaremos a continuación se pueda conectar.
```bash
curl --silent --location 'http://localhost:3000/peer-id'
```

- Response:
```bash
"12D3KooWHftS8sx3oPjPnNZVjdTS4PoDjVt2Vo56xaCDUPvhCnsE"
```
Este peer-id se genera a partir de la clave pública del material criptográfico del nodo, por lo que será diferente para cada nodo, usted utilice el que le salga a usted.


## Docker-compose
Vamos a desplegar en un docker-compose el otro nodo, un sumidero para este nodo (sumidero genérico que simplemente guarda en mongodb el evento que recibe) y un mongodb para guardar lo que pase por el sumidero. Se hace uso de un docker-compose por comodidad.
Las variables a destacar del Nodo2 son dos:
* KORE_NETWORK_ROUTING_BOOT_NODES: variable para indicar a que nodo bootstrap nos vamos a conectar para entrar a la red, cambie {{peer-id}} por el peer-id que le devolvió el Nodo1.
* KORE_BASE_SINK: servicio que hará el papel de sumidero, se le indica el nombre del schema en este caso kore-example, y el end-point donde el nodo va a enviar la información. Esto quiere decir que para todos los eventos del schema kore-example en el caso de que sean eventos que hayan pasado todos los protocolos del nodo de forma satisfactoria serán enviados al sumidero. El nodo en tiempo de ejecución puede modificar dos variables del end-point tantas veces como aparezcan:
    * {{subject-id}}: lo sustituirá por el subject-id del sujeto que esté enviando ese evento.
    * {{schema-id}}: lo sustituirá por el schema del sujeto que esté enviando ese evento.
    
    Esto es así para permitir sumideros súper flexibles y genéricos. Tanto KORE_NETWORK_ROUTING_BOOT_NODES como KORE_BASE_SINK admiten vectores, por lo cual se puede configurar más de un bootstrap y más de un sumidero.

## Nodo1 - Creación de la gobernanza:
```bash
curl --silent --location 'http://localhost:3000/event-request' \
--header 'Content-Type: application/json' \
--data '{
  "request": {
    "Create": {
      "governance_id": "",
      "schema_id": "governance",
      "namespace": "",
      "name": "gobernanza Ejemplo",
      "description": "Esta es una gobernanza para el ejemplo"
    }
  }
}'
```

- Response:
```bash
{"request_id":"JWqzQ1zJ5Ds4kGeF-ZRb4Q-5WJ5RWZvC1ff1kLmpEHTg","subject_id":"JWqzQ1zJ5Ds4kGeF-ZRb4Q-5WJ5RWZvC1ff1kLmpEHTg"}
```
Hemos obtenido una request_id y un subject_id (los suyos serán diferentes de los que salen en este ejemplo), este subject_id lo llamaremos governance-id a lo largo de este ejemplo.


## Nodo2 - Obtener el controller_id:
Vamos a obtener el controller-id del Nodo2, el controller-id es el identificar de cada nodo en la gobernanza y es necesario para añadirlo como miembro, es único para cada nodo, por lo tanto el mío será diferente al suyo.
```bash
curl --silent --location 'http://localhost:3001/controller-id'
```
- Response:
```bash
"{{controller-id-Node2}}"
```

## Nodo1 - Modificación de la gobernanza
Vamos a modificar la gobernanza para añadir el schema kore-example, para que el Nodo1 sea Creator e Issuer del schema kore-example y para que el Nodo2 sea testigo de esos sujetos de trazabilidad.
```bash
curl --request POST 'http://127.0.0.1:3000/event-request' \
    --header 'Content-Type: application/json' \
    --data '{
    "request": {
        "Fact": {
            "subject_id": "{{governance-id}}",
            "payload": {
                "members": {
                    "add": [
                        {
                            "name": "Node2",
                            "key": "{{controller-id-Node2}}"
                        }
                    ]
                },
                "roles": {
                    "schema": [
                        {
                            "schema_id": "kore-example",
                            "roles": {
                                "add": {
                                    "creator": [
                                        {
                                            "name": "Owner",
                                            "namespace": [],
                                            "quantity": "infinity"
                                        }
                                    ],
                                    "issuer": [
                                    {
                                        "name": "Owner",
                                        "namespace": []
                                    }
                                    ],
                                    "witness": [
                                    {
                                        "name": "Node2",
                                        "namespace": []
                                    }
                                    ]
                                }
                            }
                        }
                    ]
                },
                "schemas": {
                    "add": [
                        {
                            "id": "kore-example",
                            "contract": "dXNlIHNlcmRlOjp7RGVzZXJpYWxpemUsIFNlcmlhbGl6ZX07CnVzZSBrb3JlX2NvbnRyYWN0X3NkayBhcyBzZGs7CgojW2Rlcml2ZShTZXJpYWxpemUsIERlc2VyaWFsaXplLCBDbG9uZSwgRGVidWcpXQpzdHJ1Y3QgRGF0YSB7CiAgICBwdWIgdGVtcGVyYXR1cmU6IGYzMiwKICAgIHB1YiBodW1pZGl0eTogdTMyLAp9CgovLyBEZWZpbmUgdGhlIGV2ZW50cyBvZiB0aGUgY29udHJhY3QuCiNbZGVyaXZlKFNlcmlhbGl6ZSwgRGVzZXJpYWxpemUsIENsb25lKV0KZW51bSBFdmVudHMgewogICAgUmVnaXN0ZXJEYXRhIHsgdGVtcGVyYXR1cmU6IGYzMiwgaHVtaWRpdHk6IHUzMiB9LAp9CgojW3Vuc2FmZShub19tYW5nbGUpXQpwdWIgdW5zYWZlIGZuIG1haW5fZnVuY3Rpb24oc3RhdGVfcHRyOiBpMzIsIGV2ZW50X3B0cjogaTMyLCBpc19vd25lcjogaTMyKSAtPiB1MzIgewogICAgc2RrOjpleGVjdXRlX2NvbnRyYWN0KHN0YXRlX3B0ciwgZXZlbnRfcHRyLCBpc19vd25lciwgY29udHJhY3RfbG9naWMpCn0KCiNbdW5zYWZlKG5vX21hbmdsZSldCnB1YiB1bnNhZmUgZm4gaW5pdF9jaGVja19mdW5jdGlvbihzdGF0ZV9wdHI6IGkzMikgLT4gdTMyIHsKICAgIHNkazo6Y2hlY2tfaW5pdF9kYXRhKHN0YXRlX3B0ciwgaW5pdF9sb2dpYykKfQoKZm4gaW5pdF9sb2dpYyhfc3RhdGU6ICZEYXRhLCBjb250cmFjdF9yZXN1bHQ6ICZtdXQgc2RrOjpDb250cmFjdEluaXRDaGVjaykgewogICAgY29udHJhY3RfcmVzdWx0LnN1Y2Nlc3MgPSB0cnVlOwp9CgoKZm4gY29udHJhY3RfbG9naWMoCiAgICBjb250ZXh0OiAmc2RrOjpDb250ZXh0PERhdGEsIEV2ZW50cz4sCiAgICBjb250cmFjdF9yZXN1bHQ6ICZtdXQgc2RrOjpDb250cmFjdFJlc3VsdDxEYXRhPiwKKSB7CiAgICBsZXQgc3RhdGUgPSAmbXV0IGNvbnRyYWN0X3Jlc3VsdC5maW5hbF9zdGF0ZTsKICAgIG1hdGNoIGNvbnRleHQuZXZlbnQgewogICAgICAgIEV2ZW50czo6UmVnaXN0ZXJEYXRhIHsKICAgICAgICAgICAgdGVtcGVyYXR1cmUsCiAgICAgICAgICAgIGh1bWlkaXR5LAogICAgICAgIH0gPT4gewogICAgICAgICAgICBpZiB0ZW1wZXJhdHVyZSA8IC0yMF9mMzIgfHwgdGVtcGVyYXR1cmUgPiA2MF9mMzIgfHwgaHVtaWRpdHkgPiAxMDAgewogICAgICAgICAgICAgICAgcmV0dXJuOwogICAgICAgICAgICB9CiAgICAgICAgICAgIHN0YXRlLmh1bWlkaXR5ID0gaHVtaWRpdHk7CiAgICAgICAgICAgIHN0YXRlLnRlbXBlcmF0dXJlID0gdGVtcGVyYXR1cmU7CiAgICAgICAgfQogICAgfQogICAgY29udHJhY3RfcmVzdWx0LnN1Y2Nlc3MgPSB0cnVlOwp9Cg==",
                            "initial_value": {
                                "temperature": 0.0,
                                "humidity": 0
                            }
                        }
                    ]
                }
            }
        }
    }
}'
```

## Nodo1 - Aprobación del evento de Fact
Acabamos de emitir un evento de Fact para una gobernanza, como hemos explicado anteriormente estos eventos requieren de aprobación, el único aprobador actualmente es el owner de la gobernanza, es decir el Nodo1. Vamos a sacar la petición pendiente de aprobar para la gobernanza:
```bash
curl --silent --location 'http://localhost:3000/approval-request/{{governance-id}}'
```

Una vez verificada la modificación de la gobernanza que se quiere llevar acabo, vamos a aceptar la petición de aprobación:
```bash
curl --location --request PATCH '127.0.0.1:3000/approval-request/{{governance-id}}' \
--header 'Content-Type: application/json' \
--data '"Accepted"'
```

-Response
```bash
"The approval request for subject JWqzQ1zJ5Ds4kGeF-ZRb4Q-5WJ5RWZvC1ff1kLmpEHTg has changed to RespondedAccepted"
```
Cuando se cumpla el quorum de la aprobación el evento de Fact pasará a validación y finalmente a distribución. Al ser el Nodo1 el único aprobador con su aprobación basta para cumplir el quorum.

## Nodo1 - Obtener el controller_id:
Vamos a obtener el controller-id del Nodo1, necesitamos el controller-id para añadirlo en la autorización, de esta forma si se hace una actualización manual en el Nodo2 utilizará a ese nodo como testigo para pedirle la copia.
```bash
curl --silent --location 'http://localhost:3000/controller-id'
```
- Response:
```bash
"ELJAC3vJUS1pZJ4eqyNDO0X8oNxm5wZ2_lcq39RH_9pA"
```

## Nodo2 - Autorización de la gobernanza
En este punto el Nodo2 ya es miembro de la gobernanza, pero no tiene la copia de la gobernanza, hay que autorizarla, esto es así para evitar que nodos maliciosos nos añadan a gobernanza que no queremos estar y nos puedan realizar un ataque de denegación de servicio.
```bash
curl --location --request PUT '127.0.0.1:3001/auth/{{governance-id}}' \
--header 'Content-Type: application/json' \
--data '["{{controller-id-Node1}}"]'
```

Ya está la gobernanza autorizada en el Nodo2, ahora podemos pedir la gobernanza de forma manual o esperar a que se emita otro evento en la gobernanza para recibir la copia y ahí actualizarnos de forma automática. Vamos a actualizarnos de forma manual ya que en este tutorial no vamos a emitir más eventos en la gobernanza.
```bash
curl --location --request POST '127.0.0.1:3001/update/{{governance-id}}' \
--header 'Content-Type: application/json'
```

Si todo fue bien podemos ver en ambos nodos el estado de la gobernanza
```bash
curl --location '127.0.0.1:3001/state/{{governance-id}}' \
--header 'Content-Type: application/json'
```

Y el historial de eventos
```bash
curl --location '127.0.0.1:3001/events/{{governance-id}}' \
--header 'Content-Type: application/json'
```

## Nodo1 - Creación del sujeto de trazabilidad
Vamos a crear un sujeto de trazabilidad para el esquema que añadimos anteriormente.
```bash
curl --silent --location 'http://127.0.0.1:3000/event-request' \
--header 'Content-Type: application/json' \
--data '{
  "request": {
    "Create": {
      "governance_id": "{{governance-id}}",
      "schema_id": "kore-example",
      "namespace": ""
    }
  }
}'
```
- Response:
```bash
{"request_id":"JO-5HYYhU2NegOndUk7jPiBymfApK-F0o-hOXSlrcmKM","subject_id":"JO-5HYYhU2NegOndUk7jPiBymfApK-F0o-hOXSlrcmKM"}
```

Hemos obtenido una request_id y un subject_id, este subject_id será el identificador de nuestro sujeto de trazabilidad. LLegados a este punto ya está creado nuestro sujeto, ahora vamos a emitir algunos eventos de Fact para el sujeto de trazabilidad.


## Nodo1 - Emitir eventos
Vamos a emitir un evento con una temperatura y una humedad
```bash
curl --request POST 'http://localhost:3000/event-request' \
    --header 'Content-Type: application/json' \
    --data-raw '{
    "request": {
        "Fact": {
            "subject_id": "{{subject-id}}",
            "payload": {
                "RegisterData": {
                    "temperature": -5.33,
                    "humidity": 48
                }
            }
        }
    }
}'
```

## Nodo1 Nodo2 - Sujeto modificado
Vamos a ver el estado actual del sujeto, este estado se podrá ver en el nodo dueño del sujeto (Nodo1) y en los nodos testigos de ese esquema(Nodo2)
```bash
curl --location '127.0.0.1:3000/state/{{subject-id}}' \
--header 'Content-Type: application/json'
curl --location '127.0.0.1:3001/state/{{subject-id}}' \
--header 'Content-Type: application/json'
```

Y el historial de eventos
```bash
curl --location '127.0.0.1:3000/events/{{subject-id}}' \
--header 'Content-Type: application/json'
curl --location '127.0.0.1:3001/events/{{subject-id}}' \
--header 'Content-Type: application/json'
```

## Sumidero
Vamos a consultar los eventos que han pasado por el sumidero, el sumidero de ejemplo está preparado para consultar lo que se ha registrado, también se podría atacar directamente al mongodb con mongodb compas o entrando en la base de datos por consola.
```bash
curl --location --request GET 'localhost:5050/sink/{{subject-id}}' \
--header 'Content-Type: application/json'
```

- Response:
```bash
[
  {
    "_id": {
      "$oid": "681c7e48645af331f6738d9a"
    },
    "RegisterData": {
      "humidity": 48,
      "temperature": -5.33
    }
  }
]
```