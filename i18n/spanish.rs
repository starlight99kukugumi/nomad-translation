// Comments and entries are sync from the english version, it's not
// possible to have language specific comments at the moment.
// You can use this entry to make a specific comment
language_note ""

// comments with "ICON FIT" should be short, ideally < 10 characters

// When in doubt, leave an empty string, it will fallback to english
// Some terms should probably be left untranslated
// For sure: Voxel, Matcap, DynTopo, PBR, Dyntopo
// Not sure: Roughness/Metalness? Mesh? Sub? tool names? etc

// ----------------------------------------------
// general stuffs

// Popup question, confirm? [yes/cancel] [ok/cancel] [delete/cancel]
confirm "¿Confirmar?"
yes "Sí"
ok "Ok"
delete "Eliminar"
cancel "Cancelar"

// feature: Auto / Off / On
on "On"
off "Off"
auto "Auto"

// coordinate
X "X"
Y "Y"
Z "Z"

advancedSettings "Avanzado"
notSaved "Estas opciones no se guardan en la configuración."

// generic warning when there is no mesh selected
noSelectedMesh "No hay malla seleccionada."

// generic warning when only one mesh needs to be selected
multipleObjectWarning "Se seleccionaron varias mallas, selecciona solo una malla."

// ----------------------------------------------

// when you launch the app and there is missing Nomad/data files
loading.reprocess "Faltan miniaturas, reprocesamiento de archivos... ($0/$1)

$2"

// main pbr channel
baseColor "Color"
roughness "Aspereza"
metalness "Metalicidad"

// ----------------------------------------------
// about
about.minify "Minimizar la interfaz de usuario"
about.minify.help "También puedes pulsar la pantalla con 4 dedos, si el dispositivo lo admite."
about.turntable "Placa giratoria"
about.turntableSpeed "Velocidad de la placa giratoria"
about.credits "Créditos"
about.creditsOpenSource "Código abierto"
about.creditsArts "Matcaps y Hdris"
about.languages "Idiomas"
about.languages.help "El archivo de traducción está disponible en github.com/stephomi/nomad-translation"
about.openUrl "¿Abierto?"
// nomad
about.website "Sitio web"
about.forum "Foro"
about.manual "Manual"
about.mail "Soporte"
// social
about.twitter "Twitter"
about.instagram "Instagram"
about.facebook "Facebook"
about.discord "Discord"

// ----------------------------------------------
// alert
alert.hole.nothing "¡El objeto no tiene agujeros!"
alert.shape.notVisible "¡La malla actual es invisible!"
alert.trim.nothing "Nada que recortar."
alert.trim.full  "Cancelar recorte: la malla está completamente recortada."
alert.mask.noExtract "¡Nada que extraer!"
alert.mask.noSplit "¡Nada que dividir!"
alert.view.disabled "Funciones deshabilitadas en el modo de vista:"
alert.view.disabled.widgetPrimitive "Widgets primitivos"
alert.separate.fail "No se pudo separar: ¡el objeto tiene una sola parte!"
alert.voxelRemesh.success "¡Malla reforzada!"
alert.voxelRemesh.empty "Cancelar refuerzo de malla: La malla resultante no tiene caras."
alert.voxelRemesh.invalidInput "¡Entrada no válida!"
alert.matrix.clone "El objeto se duplicará"
alert.gizmo.usePivot "Utilice pivote personalizado."
alert.gizmo.useAuto "Utilice pivote automático."
alert.gizmo.editPivot "Modo de edición de pivote."
alert.gizmo.editObject "Modo de edición de objetos."
alert.dynamic.enable "Topología dinámica activa"
alert.dynamic.disable "Deshabilitar la topología dinámica"
alert.colorPicker "Arrastra el dedo sobre la malla para elegir un color."
alert.backgroundTransform "Pulsa para salir del modo de transformación."
alert.camera.resetView "Restablecer vista"
alert.camera.snapView "Vista instantánea"
alert.mask.show "Mostrar máscara"
alert.mask.hide "Ocultar máscara"
alert.selection.lock "Selección de bloqueo"
alert.selection.unlock "Selección de desbloqueo"
alert.selection.isolate "Selección de aislado"
alert.selection.showAll "Mostrar todo"
alert.quickSave "Guardando..."
alert.forceShowPainting.fill "Mostrar pintura activada, se utilizó [Pintar todo]."
alert.forceShowPainting.tool "Mostrar pintura activada, el objeto fue pintado."
alert.multiresLost "¡La multirresolución se perderá!"
alert.rangeWarning "El valor de detalle es alto y puede requerir mucha memoria"
// autosave popup
alert.autoSave.auto "Guardar automáticamente en... $0s"
// bottom warning
alert.warning.needLayer "La herramienta actual requiere una capa activa."
alert.warning.paintingHidden "Pintura oculta: muéstrela de nuevo en el panel de configuración."
alert.warning.noPartialWireframe "El dibujo parcial se desactiva cuando se muestra el esquema de página."
// bottom tip
alert.tip.shapeOrthographic "Considera la posibilidad de utilizar una cámara ortográfica si quieres evitar la distorsión de tronco de perspectiva al utilizar el proyector de pantalla."
// undo
alert.state.trial "Deshacer cancelación: versión de prueba"

// ----------------------------------------------
// background
background "Fondo"
background.color "Color"
background.environment "Entorno"
background.blur "Borroso"
background.exposure "Exposición"

background.imageEnable "Imagen de referencia"
background.imageOverlay "Superposición"
background.imageAlpha "Alfa"
background.imageReset "Restablecer configuración"
background.imageTransform "Transformar"
// transform
background.imageX "Posición X"
background.imageY "Posición Y"
background.imageRotation "Rotación"
background.imageScale "Escala"

// ----------------------------------------------
// camera
camera "Cámara"
// saved views
camera.updateView "¿Actualizar el punto de vista?"
camera.addView "Añadir vista"
camera.focusOn "Centrar"
// projection
camera.projection "Proyección"
camera.orthographic "Ortográfica"
camera.perspective "Perspectiva"
camera.fov "Campo de visión vertical"
camera.focal "distancia focal $0 mm (sensor de 35 mm)"
// orbit
camara.orbitar "Modo órbita"
camera.orbit.help "La bola de desplazamiento te da más grados de libertad. También puedes rodar la cámara con 2 dedos."
camera.trackball "Bola de desplazamiento"
camera.turntable "Placa giratoria"
// speed
camera.speed "Velocidad"
camera.rotation "Rotación"
camera.panning "Hacer panorámica"
camara.zoom "Zoom"
// misc
camera.resetView "Restablecer vista"
camara.SnapView "Capturar vista"
// interaction
camera.pivot "Pivote"
camera.doubleTapMesh "Pulsar dos veces en la malla"
camera.doubleTapBackground "Pulsar dos veces en el fondo"
camera.doubleTapPivot "Pulsar dos veces para actualizar"
camera.doubleTapPivot.help "Actualiza el pivote de rotación al pulsar dos veces la superficie del objeto."
camera.airPivot "Pivote aéreo"
camera.airPivot.help "Permite un nuevo pivote incluso al hacer zoom fuera de la superficie del objeto."
camera.autoPivot "Iniciar con gesto en cámara"
camera.autoPivot.help "Actualiza el pivote cuando comiences a interactuar con la cámara."
camera.doubleTapFocus "Centrar"
camera.doubleTapFocus.help "Al pulsar dos veces la malla, la cámara se desplazará y enfocará en el punto seleccionado."
camera.doubleTapFocusSelection "Centrar en la selección"
camera.doubleTapFocusSelection.help "Al pulsar dos veces en el fondo, céntrate en la malla seleccionada en lugar de en toda la escena."

// toolbox context, only a few tools are display in some cases
// (solo visible en el modo de caja de herramientas expandida)
context.multiselection "Multiselección"
context.triplanar "Triplanar"
context.primitive "Primitivo"

// scene and layer lists
curve.preset "Ajustes prestablecidos"
curve.custom "Personalizar"

// ----------------------------------------------
// debug
debug.uvPrimitive.warning "Deshabilita esta opción si no necesitas UV (memoria adicional)."
debug.uvPrimitive "Mantén los UV primitivos"
debug.uvPrimitive.help "Por ahora, solo se admiten Box y Sphere.

En el futuro se admitirán otros tipos."
debug.uvNormalize "Normalizar UV"
debug.uvNormalize.help "Nomad normalizará los UV dentro del mosaico [0-1]."
debug.uvBFF "Añadir UV de BFF"
debug.uvBFF.help "Agrega un método alternativo de desenvolver (primer aplanamiento de límites).

Ten en cuenta que BFF producirá superposiciones si tu topología de malla es diferente de un disco o una esfera."
debug.logs "Registros"
debug.heightmap "Mapa de alturas"
debug.graphics "Gráficos"
debug.thumbnails "Crear miniaturas de tienda"

// scene and layer lists
expandList "Interfaz de usuario: Expandir lista"
expandList.help "Solo una opción de interfaz de usuario para una administración de listas más fácil."

// ----------------------------------------------
// file
file.project.empty "¡Aún no tienes ningún proyecto guardado!"
file.project.unsaved "¡Cambios no guardados!"
file.project.loseUnsaved "¡Perderá los cambios no guardados!"
file.project.lastManualSave "Vista previa de lo último guardado manualmente"
file.project.trialNoOpen "Versión de prueba: ¡No podrás volver a abrir el proyecto actual!"
file.project.trialOnlyOpen "Versión de prueba: ¡Solo puedes volver a abrir el proyecto actual!"

// ----------------------------------------------
// project
file.project "Proyecto"
file.project.save "Guardar"
file.project.save.confirm "¿Guardar $0?"
file.project.saveAs "Guardar como"
file.project.saveAs.confirm "¿Sobrescribir $0?"
file.project.open "Abrir"
file.project.open.confirm "¿Abrir $0?"
file.project.add "Añadir a la escena"
file.project.add.confirm "¿Añadir $0 a la escena?"
file.project.new "Nuevo"
file.project.new.confirm "¿Crear nueva escena?"
file.project.delete "Eliminar"
file.project.delete.confirm "¿Eliminar $0?"
file.project.delete.confirmActive "¿Eliminar $0?

¡Este es el proyecto activo actual!"
file.project.delete.confirmOk "¿Estás seguro?"
file.project.rename "Renombrar"

// autosave
file.project.autoSave "Guardado automático del projecto"
file.project.autoSave.confirm "¿Desactivar el guardado automático?"
file.project.autoSave.help "Guarda el proyecto en un archivo separado a intervalos regulares.
El archivo de guardado automático se puede encontrar en:

$0"
file.project.autoSave.popup "Tiempo de espera emergente"
file.project.autoSave.minutes "Temporizador emergente"
file.project.autoSave.delete "Descartar auto guardado"
file.project.autoSave.delete.confirm "¿Confirmar?"

// import
file.import.title "Importar"
file.import.title.help "Formato compatible:
- Wavefront (.obj)
- glTF 2.0 (.glb .gltf)
- STL (.stl)"
file.importOpen "Abrir"
file.importOpen.confirm "¿Importar un nuevo archivo?"
file.import.add "Añadir a escena"
file.import.add.confirm "¿Importar un nuevo archivo?"

file.exportSelection "Exportar selección únicamente"
file.exportSelection.help "Exportar la malla seleccionada actual en lugar de toda la escena."
file.convertToQuad "Reconstruir quad"
file.convertToQuad.help "Reconstruye quads a partir de triángulos emparejando triángulos (si son adyacentes en los archivos)."

// export
file.export.title "Exportar"
file.export.title.help "Si es posible, usa la exportación glTF, ya que admite más funciones que otros formatos."

// generic export
file.export.texture "Exportar texturas"
file.export.texture.help "Esta opción no integra colores de vértices en texturas.

Solo reexporta texturas en caso de que ya estuvieran presentes en un archivo importado."
file.export.normal "Exportar normales"
file.export.normal.help "Marca esta opción si deseas abrir el archivo en otro software.

Nomad siempre ignora las normales, ya que las volverá a calcular."

// gltf
file.export.gltf "Exportar glTF 2.0"
file.export.gltfLayer "Exportar capas"
file.export.gltfLayer.help "Exportar capas como transformaciones. Oficialmente compatible con glTF, por lo que también debería funcionar en otros softwares."
file.export.gltfLayerPaint "Exportar pintura de capa"
file.export.gltfLayerPaint.help "Exportar pintura de capa. Por lo general, es ignorado por otros softwares."
file.export.gltfLayerNomad "Codificación de capa Nomad"
file.export.gltfLayerNomad.help "Puedes hacer que el archivo sea más pequeño escribiendo la capa de pintura en 8 bits en lugar de 16 bits.

Esta codificación solo funciona en Nomad, ya que utilizará pintura absoluta.
Otros softwares tendrán un resultado incorrecto, ya que glTF requiere una codificación relativa."
file.export.gltfColor0 "Exportar colores de vértices"
file.export.gltfColor0.help "Exportar colores de vértices. Oficialmente compatible con glTF, por lo que también debería funcionar en otros softwares."
file.export.gltfColor1 "Exportar pintura PBR"
file.export.gltfColor1.help "Exportar rugosidad, metalidad y pintura de máscaras. Esto será ignorado por otros softwares."

// obj
file.export.obj "Exportar OBJ"
file.export.objWarning "Se perderán las capas y la pintura adicional (rugosidad, metalicidad y máscara)."
file.export.objColorAppend "Exportar colores de vértices"
file.export.objColorAppend.help "Agregar información de color después de los vértices

Algunos softwares 3D podrán leerlo, pero no todos."

// stl
file.export.stl "Exportar STL"
file.export.stlWarning "Se perderán las capas y la pintura pintura (rugosidad, metalicidad y máscara)."
file.export.stlColor "Exportar colores de vértices"
file.export.stlColor.help "Algunos softwares 3D podrán leerlo, pero no todos."
file.export.stlAscii "Por defecto, el formato es binario.

Puedes exportarlo a formato de texto (ASCII), pero el archivo será más grande."

file.settings.title "Configuración"
file.settings.title.help "La mayoría de las configuraciones de la aplicación se guardan aquí (cámara, interfaz, etc.).

Algunos recursos se guardan por separado y de forma automática como:
- HDRs
- Matcaps
- Alfas
- Texturas
- Fondos
- Proyectos

Por el momento, la configuración de la brocha no se puede guardar, pero se planea una administración personalizada de la brocha."

// settings
file.settings.reset "Restablecer a los valores predeterminados"
file.settings.reset.confirm "¿Restablecer todas las configuraciones?

Los proyectos, alfas, matcaps, HDRIs y fondos no se ven afectados."

// materials
file.materials "Biblioteca de materiales"
file.materials.reset "Restablecer a predeterminado"
file.materials.reset.confirm "¿Restablecer la biblioteca de materiales?"

// tools
file.herramienta "Ajustes prestablecidos de herramientas"
file.tools.reset "Restablecer a predeterminado"
file.tools.reset.confirm "¿Restablecer biblioteca de herramientas?"

// render
file.render "Render"
file.render.showInterface "Mostrar interfaz"
file.render.renderRatio "Render proporción"
file.render.renderRatio.help "Un valor de 1.0 significa que Nomad se renderizará con la misma resolución que el tamaño de imagen solicitado a continuación.

Utiliza esta opción si no puedes renderizar a una resolución determinada (se bloquea por falta de memoria)."
file.render.help "Render proporción"
file.render.size "Tamaño final"
file.render.size.custom "Personalizar"
file.render.screenResolution "pantalla"
file.render.export "Exportar png"
file.render.width "Ancho"
file.render.height "Altura"
file.render.warn "¡La resolución de exportación es alta ($0x$1)!

Asegúrate de guardar el proyecto en caso de que el dispositivo se quede sin VRAM y se bloquee."
file.render.transparent "Fondo transparente"
file.render.transparent.help "Esta opción puede ser útil si quieres insertar la malla en un software de creación 2D.

Por el momento, no se admite la transparencia parcial de objetos."

// ----------------------------------------------
// gesture menu
gesture.useGlobal "Utilizar configuración global"
gesture.useGlobal.help "De forma predeterminada, las herramientas comparten los mismos ajustes de presión.

Desmarca esta opción si quieres la configuración de presión específica para esta herramienta."

gesture.pressure "Presión"
gesture.pressureTitle "Presión ($0)"
gesture.pressure.noTool "Esta herramienta no usa presión de lápiz."
gesture.pressure.noGrab "El tipo de pincel de agarre ignorará los ajustes de presión."
gesture.pressure.radius "Radio"
gesture.pressure.intensity "Intensidad"
gesture.pressure.useRadius "Activo"
gesture.pressure.useIntensity "Activo"
gesture.pressure.curveRadius "Radio"
gesture.pressure.curveIntensity "Intensidad"

gesture.cameraInteraction "Cámara:"
gesture.sculptInteraction "Esculpir:"
gesture.interaction.fingerAndStylus "Dedo y Stylus"
gesture.interaction.finger "Dedo"
gesture.interaction.stylus "Stylus"

gesture.fingerLighting "Rotar la iluminación (3 dedos)"
gesture.fingerLighting.help "Arrastra 3 dedos horizontalmente sobre el lienzo para rotar el entorno, las luces y el matcap."
gesture.fingerRadius "Radio de la herramienta de edición (3 dedos)"
gesture.fingerRadius.help "Arrastra 3 dedos verticalmente para editar el radio."

gesture.fingerSmooth "El dedo siempre suaviza"
gesture.unknownPressure "Permitir presión no reconocida"
gesture.unknownPressure.help "Marca esta opción si la presión no funciona con el lápiz o si necesitas presionar con el dedo."

// pencil
gesture.pencilAction.none "Ninguno"
gesture.pencilAction.smooth "Suave"
gesture.pencilAction.alt "Añadir/Restar"
gesture.pencilAction.android "Botón de lápiz"
gesture.pencilAction.android.help "Experimental"
gesture.pencilAction.ios "Pulsar dos veces con el lápiz"
gesture.pencilAction.ios.help "Solo activo para el Apple Pencil de 2.ª gen."

// history
gesto.historia "Atajos del historial"
gesture.history.help "- Deshacer: pulsa con 2 dedos
- Rehacer: pulsa con 3 dedos
- Deshacer/Rehacer: mantén 2/3 dedos hacia abajo (continuo)"

// size rejection
gesture.useSizeRejection "Utilizar rechazo de tamaño"
gesture.useSizeRejectionConfirm "Asegúrate de desactivar esta opción si tienes problemas para interactuar con el lienzo"
gesture.useSizeRejection.help "Rechaza la entrada si el área de contacto es mayor que este valor.

Es posible que no funcione en todos los dispositivos."
gesture.sizeRejection "Umbral de tamaño máximo"
// help
gesture.interaction.title "Gesto"
gesture.interaction.title.help "Estas opciones son siempre globales."

// ----------------------------------------------
// history
history "Historial"
history.root "Raíz"
history.undoConfirm "¿Confirmas deshacer todas estas operaciones?"
history.undoWarning "Si realizas una edición después, es posible que pierdas muchos cambios."
history.stack "Pila"
history.limitSize "Límite de historial (Mb)"
history.limitSize.help "Tamaño máximo (en Mb) del historial.

El historial se actualizará en la siguiente operación registrada."
history.limitStack "Límite de pila"
history.limitStack.help "Número máximo de operaciones que la aplicación puede mantener.

El historial se actualizará en la siguiente operación registrada."
history.rangeProtect "Protección de rango"
history.rangeProtect.help "Si retrocedes mucho en el historial, aparecerá un cuadro de diálogo de confirmación antes de deshacer las operaciones."
history.restoreCamera "Restaurar cámara"
history.restoreCamera.help "Habilita esta opción para restaurar el punto de vista de la cámara guardado al deshacer/rehacer una acción."
// display undo/redo
history.state.undo "Deshacer: $0"
history.state.redo "Rehacer: $0"
history.state.symmetrySplit "División de simetría"
history.state.voxelRemesh "Reforzar malla de Voxel"
historia.estado.Malla de superficie "Reforzar malla de superficie"
// state multires
history.state.multiresToDynamic "De multires a dinámico"
historia.estado.Multinivel "Cambio de resolución"
history.state.multiresSubdivide "Subdividir"
history.state.multiresReverse "Reversión"
history.state.multiresDeleteLower "Eliminar inferior"
history.state.multiresDeleteHigher "Eliminar superior"
// mesh
history.state.meshDynamicToStatic "De dinámico a estático"
history.state.meshStaticToDynamic "De estático a dinámico"
history.state.meshSymmetryUpdate "Actualización de simetría"
history.state.meshMatrixUpdate "Actualización de matriz"
history.state.meshVisibility "Visibilidad"
historia.estado.Material de malla "Cambio de material"
// state scene
history.state.sceneAddRemove "Escena"
history.state.sceneMeshOrder "Orden de la malla"
// state layer
history.state.layerOrder "Mover orden de capa $0"
history.state.layerMergeRedo "Separar la capa $0"
history.state.layerCreate "Crear capa $0"
history.state.layerDelete "Eliminar capa $0"
history.state.layerMerge "Fusionar capa $0"
history.state.layerHide "Ocultar capa $0"
history.state.layerShow "Mostrar capa $0"
history.state.layerSelect "Seleccionar capa $0"
history.state.layerUnselect "Deseleccionar capa $0"
history.state.layerName "Nombre de la capa $0"
history.state.layerFactor "Factor de capa $0"
history.state.layerFactorOffset "Factor de desplazamiento de capa $0"
history.state.layerFactorColor "Factor de color de capa $0"
history.state.layerFactorRoughness "Factor de rugosidad de capa $0"
history.state.layerFactorMetalness "Factor de metalicidad de capa $0"
// state light
historia.estado.lightVisible "luz $0 visible"
historia.estado.Intensidad de luz "Intensidad de luz $0"
history.state.lightColor "Color de luz $0"
history.state.lightPosition "Posición de luz $0"
historia.estado.Sombra de luz "Sombra de luz $0"
historia.estado.lightBias "Sesgo de sombra de luz $0"
history.state.lightAttachment "Fijación de luz $0"
history.state.lightAdd "Añadir luz $0"
history.state.lightDelete "Eliminar luz $0"
history.state.lightCopy "Copiar luz $0"
history.state.lightMove "Mover luz $0"
history.state.lightType "Tipo de luz $0"
history.state.lightSpotAngle "Ángulo de foco de luz $0"
history.state.lightSpotSoftness "Suavidad puntual de luz $0"
// state view
history.state.viewAdd "Añadir vista $0"
history.state.viewMove "Mover vista $0"
history.state.viewDelete "Eliminar vista $0"

// ----------------------------------------------
// interface
interface "Interfaz"

// bottom buttons
interface.bottomButtons "Añadir accesos directos (abajo)..."

// colors
interface.colors "Colores principales"
interface.colorSelect "Widget de color"
interface.colorBase "Base de color"
interface.colorBaseTransparent "Panel de control"
interface.panelTransparent "Panel transparente"
interface.blurFactor "Fuerza de difuminado"

// color preset
interfaz.Conjuntos de colores "Ajustes prestablecidos de color"
interface.presetBlurRed "Rojo"
interface.presetBlurBlue "Azul"
interface.presetBlurGreen "Verde"
interface.presetBlurYellow "Amarillo"
interface.presetBlackWhite "Negro y blanco"
interface.presetWhiteBlack "Blanco y negro"
interface.presetLividOrange "Lívido y naranha"
interface.presetCardboard "Cartón"
interface.presetDefault "Predeterminado"

// style
interface.style "Estilo"
interface.resetAll "Restablecer interfaz"
interface.resetAll.confirm "¿Restablecer la configuración de la interfaz?"
interface.flipTop "voltear superior"
interface.flipBottom "voltear inferior"
interface.flipMiddle "voltear medio"
interface.showTooltips "Mostrar información sobre herramientas"
interface.showTooltips.help "Esta es una información sobre herramientas."
interface.materialPreview "Vista previa del Selector de materiales"
interface.toolboxHide "Ocultar automáticamente la caja de herramientas"
interface.toolboxHide.help "Activa esta opción si quieres ocultar la caja de herramientas."
interface.toolboxMaxColumn "Caja de herramientas de columna máxima"
interface.toolboxResetOrder "Restablecer el orden de la caja de herramientas"
interface.rounding "Redondeo"
interface.curveToolSymmetric "Widget de curva de herramienta simétrica"
interface.curveToolSymmetric.help "El widget se puede encontrar en el panel de herramientas en la opción de disminución."
interface.scale "Escala general"
interface.cursorStep "Espaciado vertical"
interface.panelWidth "Ancho del panel"
interface.fontScale "Escala frontal"

// ----------------------------------------------
// layer sub menu
layer.action "Acción"
layer.name "Nombre"
layer.delete "Eliminar"
layer.move "Mover"
layer.duplicate "Duplicar"
layer.mergeDown "Fusionar hacia bajo"
layer.factors "Factores de canal"
layer.offsetFactor "Posición"
layer.colorFactor "Color"

// ----------------------------------------------
// layers menu
layers.addLayer "Añadir capa"
layers.addLayerTrial "La versión de prueba está limitada a 1 capa por malla."
layers.title "Capas"
layers.title.help "Las capas pueden registrar desplazamientos de posición y pintura, que puede ser útil para flujos de trabajo no lineales.
Por ejemplo, se puede experimentar con diferentes expresiones faciales sin depender de la pila de historial para deshacer los cambios.

Para pintar datos, las capas se ordenan de arriba hacia abajo; por lo tanto, las capas superiores enmascararán las inferiores.

Para resolver la opacidad de la capa, todos los datos de pintura (color, rugosidad, metalidad) comparten la misma máscara.
Puedes restablecer parte de esta máscara (y, por lo tanto, la influencia de la capa) utilizando la herramienta 'DelLayer'."
layers.primitive "Las capas no están disponibles para las primitivas."
layers.baseSelected "Ninguno"

// ----------------------------------------------
// light sub menu
light "Luz"
light.intensity "Intensidad"
light.attachment "Adjunto"
light.attachment.fixed "Fijado"
light.attachment.camera "Cámara"
light.attachment.environment "Entorno"
light.attachment.help "-- Fijado
La orientación de la luz no cambiará.

-- Cámara
La orientación de la luz depende de la vista de la cámara."
light.type "Tipo"
light.type.directional "Direccional"
light.type.spot "Marca"
light.type.point "Punto"
light.spotAngle "Ángulo de cono"
light.spotSoftness "Suavidad"
light.shadowCast "Sombra"
light.shadowNormalBias "Sesgo normal"
light.visible "Mostrar"
light.resetPosition "Volver a centrar"

// ----------------------------------------------
// material
material "Material"
material.addNew "Agregar nuevo"
// if the shading mode is in matcap or unlit
material.pbrRoughnessMetalnessWarning "La rugosidad y la metalicidad requieren el modo de sombreado PBR."
material.pbrReflectanceWarning "La reflectancia requiere el modo de sombreado PBR."
material.pbrRefractionWarning "La refracción requiere el modo de sombreado PBR."
material.pbrSubsurfaceWarning "La subsuperficie requiere el modo de sombreado PBR."
// refraction
material.ior "Índice de refracción"
material.paintingOverride "Anular pintura"
material.paintingOverride.help "Hay dos rugosidades en juego, la que impulsa la superficie y la que está en el interior.

Sin embargo, solo hay una rugosidad que se puede pintar, por lo que las dos rugosidades tienen los mismos valores.

Puede usar este control deslizante para anular la rugosidad de la superficie para que tenga más brillo."
material.refractionSurfaceGlossiness "Brillo de la superficie"
material.refractionSurfaceGlossiness.help "- en 0, la superficie está utilizando la rugosidad pintada
- en 1, la superficie es completamente lisa"
material.refractionInteriorRoughness "Rugosidad del interior"
material.refractionInteriorRoughness.help "- en 0, el interior está utilizando la rugosidad pintada
- en 1, el interior es completamente áspero"
material.paintGlossy "Pintura brillante"
material.paintGlossy.help "Pintará el objeto con una rugosidad y metalicidad de 0, lo que permitirá una refracción nítida.

Esto es lo mismo que ir al menú pintar y usar la función pintar todo con el color y el metal desactivados."
// absorption
material.absorptionEnable "Absorción"
material.absorptionEnable.help "Simule la luz que se absorbe cuando viaja a través del volumen.

Las partes delgadas brillarán a medida que deja pasar más luz, mientras que las áreas gruesas serán más oscuras.

El efecto depende en gran medida de la geometría de la malla, solo se utiliza una aproximación del grosor de la malla."
material.absorptionFactor "Factor"
// subsurface
material.subsurfaceDepth "Profundidad"
material.translucency "Translucidez"
material.translucency.help "Necesitas tener una luz que proyecte sombras para ver la translucidez."
// type
material.opacity "Opacidad"
material.type.opaque "Opaco"
material.type.subsurface "Subsuperficie"
material.type.subsurface.help "Para obtener el mejor resultado, puedes cambiar al modo de sombreado PBR y usar al menos una luz direccional, idealmente con un entorno tenue."
material.type.blending "Mezcla"
material.type.blending.help "Haz que la malla sea semitransparente ajustando el valor de opacidad

Ten en cuenta que, debido a las limitaciones en tiempo real, puedes tener artefactos visuales notables si tu objeto tiene una forma compleja."
material.type.additive "Aditivo"
material.type.additive.help "Haz que la malla sea semitransparente ajustando el valor de opacidad.

Este método tiende a tener menos artefactos que el método de fusión, pero el objeto será más brillante."
material.type.dithering "Tramado"
material.type.dithering.help "Haz que el objeto sea semitransparente descartando algunos píxeles de forma aleatoria."
material.type.refraction "Refracción"
material.type.refraction.help "Este modo se puede utilizar para simular material de vidrio

Debido a las limitaciones de tiempo real, la autorrefracción o la refracción multicapa es limitada."
// shadows
material.castShadows "Proyectar sombras"
material.receiveShadows "Recibir sombras"
// backface
material.twoSided "Dos caras"
material.alwaysUnlit "Siempre apagado"
material.flipCulling "Culling inverso"
// material
material.reflectance "Reflectancia"
material.reflectance.help "Controla la cantidad de reflexión que recibirá el material para materiales no metálicos

La mayoría de las veces, se debe usar el valor predeterminado (0,5, que corresponde al 4 % de luz reflejada estándar en ángulo normal)."

// ----------------------------------------------
// menu name (visible on small screen menu are collapsed)
menu.files "Archivos"
menu.scene "Escena"
menu.multires "Multires"
menu.voxel "Voxel"
menu.dyntopo "DynTopo"
menu.topology "Deci/UV..."
menu.primitive "Primitivo"
menu.render "Render"
menu.material "Material"
menu.postProcess "Posprocesado"
menu.camera "Cámara"
menu.background "Fondo"
menu.tool "Herramienta"
menu.stroke "Trazo"
menu.paint "Pintura"
menu.symmetry "Simetría"
menu.pressure "Presión"
menu.layers "Capas"
menu.settings "Configuración"
menu.interface "Interfaz"
menu.history "Historia"
menu.historySettings "Configuración"
menu.about "Acerca de"
menu.debug "Depurar"

// ----------------------------------------------
// mesh sub menu
mesh.action "Acción"
mesh.holeClose "Cerrar agujeros"
mesh.holeDetail "Detalle"
mesh.separate "Separado"
mesh.triplanarWarning "Se perderán las capas, la pintura y la multirresolución."
mesh.triplanarResolution "Resolucion"
mesh.triplanarCubic "Fuerza cúbica"
mesh.triplanarConvert "Convertir"
mesh.name "Nombre"
mesh.type "Tipo"
mesh.typeStatic "Estático"
mesh.typeMultiresolution "Multiresolución"
mesh.typeDynamic "Dinámico"

// ----------------------------------------------
// painting
paint.useGlobal "Material globacl"
paint.useGlobal.help "Si esta opción está habilitada, el material seleccionado será el mismo que el de las otras herramientas.

Ten en cuenta que solo toma en cuenta la rugosidad, la metalicidad y los ajustes de color."
paint.usePainting "Pintura de trazo"
paint.intensity "Intensidad de pintura"
paint.paintAll "Pintar todo"
paint.paintAll.help "Aplicar el material actual a la malla."
paint.paintAllForce "Forzar pintura en todo"
paint.paintAllForce.help "Aplicar el material actual a la malla.

El área enmascarada y los canales deshabilitados no se pintarán."
paint.strokePaintingTitle "Pintura ($0)"
paint.layerWarning "El enmascaramiento de canal se ignorará si intentas aplicarlo en una capa."
paint.texture.title "Textura"
paint.texture.title.help "Una imagen que coloreará tu trazo.

Ten en cuenta que compartirás la configuración de mosaico y escala del alfa."
paint.texture.warningEnable "La pintura de trazos debe estar habilitada para permitir la proyección de texturas (casilla de verificación en la parte superior)"
paint.texture.warningIgnored "¡La herramienta actual no puede usar texturas!"
paint.useAlpha "Usar trazo alfa"
paint.useAlpha.help "Uso del conjunto alfa en el menú trazo para modular la pintura."
paint.useFalloff "Utilizar trazo de disminución"
paint.useFalloff.help "Usar del set de disminución en el menú trazo para modular la pintura."

// ----------------------------------------------
// popup (for example tap on a tool, to open edit popup)
popup.save "Guardar"
popup.save.confirm "¿Confirmar guardado?"
popup.lastSave "Último guardado"
emergente.lastSave.confirmar "¿Cargar último guardado?"
popup.reset "Restablecer"
popup.reset.confirm "¿Confirmar restablecido?"
popup.clone "Clonar"
popup.rename "Renombrar"
popup.delete "Eliminar"
popup.delete.confirm "¿Confirmar eliminación?"
popup.delete.confirm.yes "Sí, eliminar"

// title when requesting input value through virtual keyboard
input.name "Nombre"
input.number "Valor"
input.hexcolor "Color hexadecimal"

// ----------------------------------------------
// postprocess
postprocess.mainEnable "posprocesado"
postprocess.quality "Calidad"
postprocess.quality.help "Activa estas opciones para mejorar la calidad en detrimento del rendimiento.

Mejorará:
- Reflexión
- Subsuperficie
- Oclusión ambiental
- Profundidad de campo"
postprocess.maxSamples "Muestras máx."
postprocess.fullResolution "Resolución completa"
postprocess.renderRatio "Renderizar resolución"
postprocess.renderRatioWarning "Anulado por el efecto Pixel Art."
postprocess.renderRatio.help "Esta opción impacta mucho en el rendimiento.
Se recomienda mantener un valor inferior a x1.25.

Esta opción no se guarda en la configuración."
// fxaa
postprocess.fxaaEnable "Suavizado (FXAA)"
// taa
postprocess.taaEnable "Suavizado (TAA)"
postprocess.taaEnable.help "Reduce el parpadeo al mover la cámara."
// ssr
postprocess.ssrEnable "Reflexión (SSR)"
postprocess.ssrPBRWarning "SSR requiere el modo de sombreado PBR."
// ssao
postprocess.ssaoEnable "Oclusión ambiental"
postprocess.ssaoRadius "Tamaño"
postprocess.ssaoFactor "Fuerza"
postprocess.ssaoBias "Sesgo de curvatura"
postprocess.ssaoBias.help "La sensibilidad del efecto depende de la curvatura de la superficie."
// dof
postprocess.dofEnable "Profundidad de campo"
postprocess.dofBlurFar "Borrosidad lejana"
postprocess.dofBlurNear "Borrosidad cercana"
postprocess.dofFocusTip "Pulsa un objeto para cambiar el punto de enfoque."
// bloom
postprocess.bloomEnable "Resplandor"
postprocess.bloomIntensity "Intensidad"
postprocess.bloomRadius "Radio"
postprocess.bloomRadius.help "Qué tan extendido está el resplandor."
postprocess.bloomThreshold "Umbral"
postprocess.bloomThreshold.help "Umbral de luminosidad para decidir si un píxel emitirá resplandor o no.
Si el valor es 0, todo tendrá resplandor."
// tone mapping
postprocess.toneEnable "Mapeo de tonos"
postprocess.toneExposure "Exposición"
postprocess.toneContrast "Contraste"
postprocess.toneSaturation "Saturación"
postprocess.toneMappingNone "Ninguno"
// curve
postprocess.curveEnable "Graduación de color"
postprocess.curve.luminance "Principal"
postprocess.curve.red "Rojo"
postprocess.curve.green "Verde"
postprocess.curve.blue "Azul"
postprocess.curveReset "Restablecer"
postprocess.curveResetAll "Restablecer todo"
// chromatic
postprocess.chromaticEnable "Aberración cromática"
postprocess.chromaticFactor "Fuerza"
// vignette
postprocess.vignetteEnable "Viñeta"
postprocess.vignetteSize "Tamño"
postprocess.vignetteHardness "Dureza"
// sharpness
postprocess.sharpnessEnable "Nitidez"
postprocess.sharpnessFactor "Fuerza"
// grain
postprocess.grainEnable "Ruido"
postprocess.grainFactor "Fuerza"
// curvature
postprocess.curvatureEnable "Curvatura"
postprocess.curvatureCavity "Cavidad"
postprocess.curvatureBump "Bump"
// pixelart
postprocess.pixelartEnable "Pixel Art"
// scanline
postprocess.scanlineEnable "Scanline"
postprocess.scanlineFactor "Factor"
postprocess.scanlineSpacing "Espaciado"

// ----------------------------------------------
// primitive (scene menu)
primitive "Primitivo"
primitive.box "Caja"
primitive.sphereCube "Esfera"
primitive.sphereUV "Esfera UV"
primitive.icosahedron "Icosaedro"
primitive.cylinder "Cilindro"
primitive.cone "Cono"
primitive.torus "Toro"
primitive.lathe "Torno"
primitive.tube "Tubo"
primitive.plane "Plano"
primitive.triplanar "Triplanar"
primitive.faceXYZ "Cara XYZ"
primitive.faceXYZ.help "Malla base sin envolver UV proporcionada por https://texturing.xyz/"
primitive.needValidate "Las primitivas deben ser validadas para ser esculpidas."

// for 3d editing in viewport
primitive.useFloatPanel "Panel dentro de la ventana gráfica"
primitive.useFloatPanel.help "Mover algunas de las opciones de primitivas directamente en la ventana gráfica."
primitive.edit "Editar"
primitive.edit.help "Permitir la edición 3D en la ventana gráfica.

Puedes desactivar esta función si deseas interactuar con el aparato o la herramienta de transformar modificando la primitiva."

primitive.mainConfig "Parámetro"
primitive.topology "Topología"
primitive.geometry "Geometría"

// common config
primitive.validate "Validar"
primitive.maxFaces "Caras máx."
primitive.maxFaces.help "El número máximo de caras que puede tener una primitiva.

Este límite solo está activo mientras la primitiva no está validada, después la salvaguarda desaparece."
primitive.linear "Subdivisión lineal"
primitive.subdivision "Postsubdivisión"

// common config
primitive.radius "Radio"
primitive.size "Tamaño"
primitive.sizeX "Tamaño X"
primitive.sizeY "Tamaño Y"
primitive.sizeZ "Tamaño Z"
primitive.division "Division"
primitive.divisionX "División X"
primitive.divisionY "División Y"
primitive.divisionZ "División Z"
primitive.angleX "Ángulo X"
primitive.angleY "Ángulo Y"
primitive.angleZ "Ángulo Z"
primitive.constantDensity "Densidad constante"
primitive.projectOnSphere "Proyecto sobre esfera"
primitive.projectOnSphere.help "Ajustar los puntos en una esfera perfecta."

// triplanar
primitive.triplanar.title "Triplanar - Configuración"
primitive.triplanar.title.help "Triplanar utiliza la información de máscara de 3 planos para rellenar una cuadrícula de Voxel que luego se poligoniza.

Si interactúas con los controles deslizantes de división o tamaño, la información de pintura se restablecerá (la suavidad está bien).

Probablemente deberías deshabilitar la simetría, ya que podría no funcionar como cabría esperar.

Puedes usar la opción 'Conectado topológicamente' en el panel máscara para pintar un plano que impacte en los otros planos."
primitive.triplanarSameSize "Mismo tamaño (cubo)"
primitive.triplanarPolish "Suavidad"
primitive.triplanarResetMask "Restablecer máscara"
primitive.triplanarReproject "Redimensionar máscara"
primitive.triplanarReproject.title "Reproyectar la máscara de plano al actualizar la configuración triplanar

Si desmarca esta opción, volverá a las máscaras esféricas predeterminadas."
primitive.isolate.all "Todo"
primitive.isolate.back "Atrás"
primitive.isolate.right "Derecho"
primitive.isolate.bottom "Inferior"
// plane
primitive.planeSameSize "Mismo tamaño (cuadrado)"
primitive.planeDisk "Disco"
// box
primitive.boxRegular "Mismo tamaño (cuba)"
// tube
primitive.tubeSnapOffset "Capturar desplazamiento"
primitive.tubeSnapOffset.help "Un valor de 1.0 es igual radio del tubo."
primitive.tubeThicknessStart "Inicio del radio"
primitive.tubeThicknessEnd "Fin del radio"
// primitive.tubeTwist "Giro"
// primitive.tubeTwistRotate "Rotación"
// primitive.tubeTwistRadius "Magnitud"
// primitive.tubeTwistOffset "Desplazamiento"
primitive.tubeSnap "Capturar"
// lathe
// torus
primitive.torusRadiusOuter "Radio exterior"
primitive.torusRadiusInner "Radio interior"
primitive.torusAngle "Ángulo"
primitive.torusAngleOffset "Desplazamiento de ángulo"
// cylinder
primitive.cylinderHeight "Altura"
// cone
primitive.coneRadius "Radio"
primitivo.coneHeight "Altura"
// hole sub menu (cylinder, tube, etc)
primitive.hole "Agujero"
primitive.hasHole "Tiene un agujero"
// both used for hole radius and main radius
primitive.radiusSync "El mismo radio"
primitive.radiusStart "Inicio del radio"
primitive.radiusEnd "Fin del radio"
primitive.editRadius "Radio"
// spline (for lathe and tube)
primitive.spline "Spline"

// common resources stuffs
resource.delete "Eliminar"
resource.import "Importar"

// ----------------------------------------------
// scene
scene.title "Escena"
scene.title.help "Cuando uses la casilla de verificación de selección, mantén presionado y arrastra el dedo para seleccionar otros objetos fácilmente."
scene.mergeSimple "Combinación simple"
scene.mergeVoxel "Combinar Voxel"
scene.voxelResolution "Resolución"
scene.subtractionTip "Resta  : ocultar malla (icono de ojo)"
scene.intersectionTip "Intersección: Todas las mallas ocultas"

// ----------------------------------------------
// settings
settings.displayTitle "Configuración de pantalla"
// wireframe
settings.wireframeTitle "Bastidor"
settings.wireframeDisplay "Bastidor"
settings.debugUV "Depurar UV"
settings.debugUV.help "Esta opción solo es relevante si el modelo tiene UV.

Mostrará la estructura metálica UV en segundo plano.

También mostrará una textura de tablero de ajedrez de color en el modelo."
// backface
settings.backfaceTitle "Dos cara"
settings.backfaceVisible "Dos cara"
settings.backfaceVisible.help "Las caras serán visibles desde ambos lados."
settings.backfaceColor "Color de cara posterior"
settings.backfaceColored "Cara posterior coloreada"
// outline
settings.outlineTitle "Contorno"
settings.outlineEnable "Contorno"
settings.outlineThickness "Espesor"
// snap cube
settings.snapCubeTitle "Capturar cubo"
settings.snapCubeDisplay "Capturar cubo"
settings.snapCubeBottom "Inferior"
settings.snapCubeLeft "Izquierdo"
// stats
settings.statsTitle "Estadísticas"
settings.statsDisplay "Estadísticas"
settings.statsRight "Derecho"
settings.statsAll "Mostrar escena completa"
// grid
settings.gridTitle "Cuadrícula"
settings.gridDisplay "Cuadrícula"
// cursor
settings.cursorWhileSculpting "Mostrar círculo mientras esculpes"
settings.cursorShowDot "Mostrar punto pequeño"
settings.cursorShowDot.help "El punto puede aparecer como el punto de pivote de la cámara o cuando estás esculpiendo."
settings.cursorShowRope "Mostrar estabilizador de cuerda"
// highlight
settings.highlightSelectionTitle "Selección de resaltado"
settings.highlightSelection "Resaltar mallas seleccionadas"
settings.highlightDuration "Duración"
// other
settings.darkenUnselected "Oscurecer mallas no seleccionadas"
settings.smoothShading "Sombreado suave"
settings.partialDraw "Dibujo parcial"
settings.partialDraw.help "¡Característica no pulida!

Úsela si estás esculpiendo una parte relativamente pequeña de una malla de polietileno alto.

Debería hacer que la escultura sea más suave, pero no debes habilitar el bastidor.

También puede agregar artefactos visuales durante los trazados"
settings.partialDrawWarning "¡No olvides desactivar esta opción si los artefactos visuales son demasiado molestos!"
settings.showPainting "Mostrar pintura"
settings.lightIcon "Iconos de luz"
settings.lightIcon.help "Muestra los iconos claros en el lienzo para que puedas seleccionarlos y editarlos directamente."
settings.holeTitle "Relleno de agujero"
settings.holeNonManifold "Rellenar espacio no topológico"
configuración.holeNonManifold.ayudar "Intenta llenar el agujero del espacio topológico.
Esta opción no se guarda en la configuración."
settings.loadGuiSettings "Mantener la configuración de la gui (al importar)"
settings.loadGuiSettings.help "Al abrir o importar un archivo de proyecto, se cargarán todas las configuraciones relacionadas con la gui integradas en el proyecto."
settings.loadObjSplitByGroup "Dividir OBJ por grupos"
settings.loadObjSplitByGroup.help "Cuando está habilitado, Nomad dividirá el grupo de vértices OBJECT en objetos separados."
settings.loadMergeLayers "Combinar capas (al importar)"
settings.loadSkipTextures "Omitir texturas (al importar)"
settings.loadKeepTopology "Mantener topología (al importar)"
settings.loadKeepTopology.help "Utilice esta opción si no desea que Nomad juegue con la topología de la malla importada.

Deshabilitará el reordenamiento de vértices/caras, la eliminación de duplicados de vértices/caras y la eliminación de vértices no utilizados."
settings.loadReverseVertices "Revertir umbral"
settings.loadReverseVertices.help "Para usar menos memoria, Nomad no guarda la resolución más baja de una malla.

Sin embargo, reconstruirá la resolución más baja si el número de vértices es menor que este umbral."
// multires
settings.multiresTitle "Multiresolución"
settings.multiresMaxVertices "Recuento máximo de vértices"
settings.multiresMaxVertices.help "Nomad no realiza una comprobación de memoria antes de la subdivisión, un recuento alto de poli puede provocar bloqueos fácilmente."
settings.multiresLowResVertices "Umbral de baja resolución"
settings.multiresLowResVertices.help "Se puede mostrar una resolución más baja de la malla cuando mueve la cámara.

Puedes aumentar este valor si deseas mostrar una resolución más alta de la malla."
// experimental
settings.experimentalTitle "Experimental"

// ----------------------------------------------
// shading
shading "Sombreado"
// main render mode
shading.pbr "Encendido (PBR)"
shading.pbr.help "En este modo, puede agregar luces (con sombras), junto con un entorno HDR.

También puede pintar metal y rugosidad, lo que permite un control más fino sobre el aspecto de tu material."
shading.matcap "Matcap"
shading.matcap.help "Matcap significa captura de material (MATerial CAPture) y se encarga de la iluminación y la información del material en una sola imagen.

Este es un modo de renderizado rápido, adecuado principalmente para esculpir en bruto."
shading.unlit "Apagado"
shading.unlit.help "Modo de sombreado de color sólido, apagado."
// textures
shading.textures "Utilizar texturas"
shading.textures.help "Por el momento, no puedes crear ni editar texturas dentro de Nomad.

Pero si importas un archivo con texturas, debería funcionar.

-- Texturas compatibles --
Opacidad: Iluminado, Matcap, apagado
Normal: Encendido, Matcap
Color: Encendido, Apagado
Emisor: Encendido
Rugosidad: Encendido
Metalidad: Encendido"
// lights
shading.lights "Luces"
shading.lights.addLight "Agregar luz"
shading.lights.pbrWarning "Las luces requieren el modo de sombreado PBR."
// environment
shading.environment "Entorno"
shading.environmentImport "Importar HDR"
shading.environmentExposure "Exposure"
shading.environmentBackgroundBlur "Borroso (fondo)"
shading.environmentRotation "Rotación"
shading.environmentRotation.help "Puedes rotar el entorno arrastrando 3 dedos horizontalmente en la ventana gráfica."
shading.environmentAttachedToCamera "Conectado a la cámara"
shading.environmentAttachedToCamera.help "Conecte el entorno a la cámara.

Obligará a que la iluminación sea consistente, lo que puede ser útil para esculpir."
// matcap
shading.matcap "Matcap"
shading.matcapRotation "Rotación"
shading.matcapRotation.help "Puedes girar el matcap arrastrando 3 dedos horizontalmente en la ventana gráfica."
shading.matcapGlobal "Utilizar el matcap global"
shading.matcapGlobal.help "Desmarca esta opción para usar un matcap diferente para esta malla en particular."

// ----------------------------------------------
// bottom shortcut buttons (ICON FIT)
shortcut.maskVisible "Máscara"
shortcut.solo "Solo"
shortcut.voxelRemesh "Voxel"
shortcut.wireframe "Alambre"
shortcut.cameraReset "Restablecer"
shortcut.cameraSnap "Capturar"
shortcut.lockSelection "Bloquear"
shortcut.perspective "Persp"
shortcut.grid "Cuadrícula"
shortcut.uv "uv"

// can be longer (customization name in Interface menu)
shortcut.voxelRemesh.long "Reforzar malla Voxel"
shortcut.wireframe.long "Bastidor"
shortcut.cameraReset.long "Restablecer cámara"
shortcut.cameraSnap.long "Captura de cámara"
shortcut.lockSelection.long "Selección de bloqueo"
shortcut.lockSelection.long.help "Cuando está habilitado, no puedes cambiar la selección pulsando una malla."
shortcut.perspective.long "Perspectiva"
shortcut.grid.long "Cuadrícula"

// ----------------------------------------------
// stat
stat.ramScene "Escena"
stat.vramScene "Escena Vram"
stat.vramRender "Renderizar Vram"
stat.vramTextures "Texturas Vram"
stat.ramHistory "Historial"
stat.ramOther "Otro"
stat.usedMemory "Memoria usada"
stat.freeMemory "Memoria libre"
stat.ram "RAM"
stat.used "Usado: $0 MB"
stat.free "Libre: $0 MB"
stat.faces "Caras"
stat.triangles "Triángulos"
stat.vertices "Vértices"
stat.quads "Quads"
stat.sceneFaces "Caras de escena"
stat.sceneVertices "Vértices de escena"

// ----------------------------------------------
// stroke
stroke "Trazo"
strokeTitle "Trazo ($0)"
stroke.useWorldRadius "Radio mundo-espacio"
stroke.useWorldRadius.help "Es compartido entre todas las herramientas."
stroke.useShareRadius "Compartir radio"
stroke.useShareRadius.help "Comparte el valor del radio entre todas las herramientas."
stroke.minSpacingAdjustIntensity "Ajusta la intensidad del espaciado"
stroke.minSpacingAdjustIntensity.help "Ajusta la intensidad de la brocha para garantizar una deformación uniforme en función de la separación entre trazos."
stroke.minSpacing "Espaciado de trazos"
stroke.minSpacing.help "Espaciado entre cada trazo, en relación con el radio de la herramienta.

Un valor más bajo permitirá un trazo más suave, pero el rendimiento se degradará."
stroke.lazySmooth "Suavidad del trazo"
stroke.lazySmooth.help "Promedia la posición de varios punteros para obtener un trazo más suave

Con valores altos, el trazo se retrasará detrás del puntero, pero eventualmente lo alcanzará."
stroke.lazyRadius "Estabilizador de cuerda perezoso"
stroke.lazyRadius.help "Los trazos se retrasarán detrás de la posición del puntero de acuerdo con una cierta distancia.

Esto se puede usar para dibujar líneas suaves."
stroke.globalSettings "Este es una configuración global"
stroke.snapRadius "Capturar radio"
stroke.snapRadius.help "Carturar el trazo si el puntero se encuentra cerca del último trazo grabado

Esto puede ser útil al dibujar líneas largas y continuas, mientras se hacen pausas frecuentes."
stroke.sculptOffset "Desplazamiento de trazo"
stroke.sculptOffset.help "Aplica un desplazamiento constante en el trazo.

Esta opción está ahí para ayudar con la pantalla pequeña cuando se usan los dedos para no cubrir el trazo."
stroke.accumulate "Acumular trazo"
stroke.accumulate.help "Si esta opción está habilitada, no hay límite en la cantidad de material que puedas agregar/eliminar por trazo."
stroke.useDynamicTopology "Permitir topología dinámica"
stroke.connectedTopology "Topología conectada"
stroke.connectedTopology.help "Esta opción solo esculpirá los vértices que están vinculados a la superficie seleccionada.

Por lo general, se utiliza para la herramienta Mover, por ejemplo, si quieres mover exclusivamente una parte que se interseca automáticamente con otra parte."
stroke.onlyFrontFace "Solo vértice frontal"
stroke.onlyFrontFace.help "Esta opción ignorará los vértices orientados hacia atrás.

Puede ser útil si quieres pintar parte de una geometría fina sin afectar al otro lado.

También funciona para esculpir, pero es posible que experimentes algunos artefactos."
stroke.onlySameSide "Solo vértice del mismo lado"
stroke.onlySameSide.help "Ignora los vértices que apuntan en la dirección opuesta a la deformación."
stroke.curveFalloff "Disminución"
stroke.onlyLasso "La configuración solo está activa para la herramienta lazo."
// alpha
stroke.alpha "Alfa"
stroke.alphaInvert "invertir valor"
trazo.alphaWrap "Mosaico"
stroke.alphaWrap.none "Ninguno"
stroke.alphaWrap.repeat "Repetir"
stroke.alphaWrap.mirror "Espejo"
stroke.alphaProject "Método"
stroke.alphaProject.surfaceContinuous "Superficie"
stroke.alphaProject.screenFixed "Proyecto de pantalla"
stroke.alphaTiling "Mosaico"
stroke.alphaScale "Escalar"
stroke.alphaScale.help "En el valor mínimo, el cuadrado alfa está dentro del radio del círculo de la herramienta."
stroke.alphaMidValue "Valores medios"
stroke.alphaMidValue.help "Valor de punto medio en el que no se produce deformación.

(Valor medio = 0)
- Negro: sin desplazamiento
- Blanco: desplazamiento positivo

(Valor medio = 0.5)
- Negro: desplazamiento negativo
- Blanco: desplazamiento positivo

(Valor medio = 1)
- Negro: desplazamiento negativo
- Blanco: sin desplazamiento"
// stroke type
stroke.strokeType "Tipo de trazo"
stroke.strokeTypeDot "Punto"
stroke.strokeTypeDrag "Arrastrar"
stroke.strokeTypeGrab "Agarre"
stroke.strokeTypeGrabRadius "Agarre - radio dinámico"
stroke.strokeTypeGrabIntensity "Agarre - intensidad dinámica"

// ----------------------------------------------
// symmetry
symmetry "Simetría"
symmetry.enable "Activado"
symmetry.plane.title "Planos"
symmetry.toolIgnore "La herramienta actual ignora la simetría."
symmetry.radial.title "Radial"
symmetry.radialX "Radial X"
symmetry.radialY "Radial Y"
symmetry.radialZ "Radial Z"
// method
symmetry.method "Método:"
symmetry.method.help "-- Local
El plano de simetría se moverá a lo largo de la malla cuando utilice una de las herramientas de transformación (rotar, traducir o aparato).


-- Mundo
El plano de simetría es fijo y no se mueve."
symmetry.methodWorld "Mundo"
symmetry.methodLocal "Local"
// flip
symmetry.flip "Voltear objeto"
// mirror
symmetry.mirror "Duplicación"
symmetry.mirror.help "Intenta volver a aplicar la simetría sin afectar a la topología.

Se ignorará la simetría radial.

Si la topología no se puede mantener porque no se considera simétrica, tendrás la opción de aplicar la duplicación."
symmetry.mirrorLeftToRight "De izquierda a derecha"
symmetry.mirrorRightToLeft "De derecha a izquierda"
symmetry.mirrorFail "No se pudo aplicar la simetría.

¿Quieres imponer la simetría duplicando la malla?"
symmetry.mirrorUseMasking "Proteger área enmascarada"
symmetry.mirrorUseMasking.help "Mantener el área enmascarada intacta.

Esta opción se ignorará con topología no simétrica (o superficie desconectada, como un par de ojos)."
// reset
symmetry.reset "Restablecer"
symmetry.resetCenterMesh "Centro de maya"
symmetry.resetCenterWorld "Centro del Mundo"
symmetry.resetDirection "Orientación"
// advanced
symmetry.showLine "Mostrar línea"
symmetry.showPlane "Mostrar plano"
symmetry.editWarning "La edición de simetría es experimental."
symmetry.edit "Editar aparato"
symmetry.edit.help "Puedes establecer libremente el plano de simetría.

Esta función es un poco experimental y probablemente nunca deberías usarla."

// ----------------------------------------------
// tools icons on the left bar (ICON FIT)
tool.dynTopo "DynTopo"
tool.symmetry "Sim"
tool.mirror "Espejo"
tool.clay "Arcilla"
tool.clay.sub "Sub"
tool.brush "Brocha"
tool.move "Mover"
tool.move.normal "Normal"
tool.drag "Arrastrar"
tool.smooth "Suavizar"
tool.smooth.relax "Relajar"
tool.mask "Máscara"
tool.mask.unmask "Quitar máscara"
tool.maskSelector "Sel. máscara"
tool.smudge "Difuminar"
tool.flatten "Aplanar"
tool.flatten.fill "Rellenar"
tool.layer "Capa"
tool.crease "Doblar"
tool.trim "Recortar"
tool.split "Dividir"
tool.project "Proyectar"
tool.inflate "Inflar"
tool.pinch "Pellizcar"
tool.nudge "Posicionar"
tool.stamp "Tapón"
herramienta.Capa transparente "Eli. capa"
tool.lassoSelect "Seleccionar"
tool.gizmo "Aparato"
tool.gizmo.auto "Auto"
tool.gizmo.editPivot "Pivote"
tool.gizmo.rotateSnap "Capturar"
tool.gizmo.local "Local"
tool.transform "Transformar"
tool.transform.move "Mover"
tool.transform.rotate "Rotar"
tool.transform.scale "Escala"
tool.transform.snap "Capturar"
tool.measure "medir"
tool.view "Ver"
tool.lathe "Torno"
tool.tube "Tubo"
tool.insert "Insertar"
tool.shape.flip "Voltear"
tool.shape.view "Ver"
tool.shape.lasso "Lazo"
tool.shape.curve "Curva"
tool.shape.polygon "Polígono"
tool.shape.path "Ruta"
tool.shape.rectangle "Rect"
tool.shape.ellipse "Elipse"
tool.shape.line "Línea"
tool.shape.closed "Cerrado"

// popup when editing sliders
tool.sliderRadius "Radio $0"
tool.sliderIntensity "Intensidad $0 %"

// ----------------------------------------------
// title
tool.settingsTitle "Configuración ($0)"

// ----------------------------------------------
// tool menu
tool.noSettings "Esta herramienta no tiene ninguna configuración específica."

// ----------------------------------------------
// clay
tool.clay.flattenOffset "Desplazamiento de aplanamiento"

// ----------------------------------------------
// crease
tool.crease.pinchFactor "Pellizcar fuerza"

// ----------------------------------------------
// layer
tool.layer.removeInfluence "Usar desplazamiento de capa actual"
tool.layer.removeInfluence.help "Esta opción solo está activa cuando hay una capa actual seleccionada.

Utilizará el desplazamiento de capa actual para limitar el desplazamiento a lo largo de los trazos."
tool.layer.noLayerSelected "Esta opción solo está disponible si se seleccionas una capa actual"

// ----------------------------------------------
// flatten
tool.flatten.warning "Estas opciones son experimentales y podrían eliminarse en el futuro"
tool.flatten.planeLockOrigin "Bloquear origen del plano"
tool.flatten.planeLockNormal "Bloquear dirección del plano"
tool.flatten.planeAverageOrigin "Promedio de origen del plano"
tool.flatten.planeAverageNormal "Promedio de dirección del plano"
tool.flatten.planeOffset "Desplazamiento del plano"

// ----------------------------------------------
// smooth
tool.smooth.stickyBorder "Vértice pegajoso  en el borde"

// ----------------------------------------------
// paint
tool.paint "Pintura"
tool.paint.erase "Eliminar"
tool.paint.depthFilter "Filtrado profundo"
tool.paint.layerFilter "Filtrado de capas"
tool.paint.layerFilter.help "Utiliza esta opción si solo quieres volver a pintar el área ya pintada de una capa."

// ----------------------------------------------
// masking
tool.mask.clear "Eliminar"
tool.mask.invert "Invertir"
tool.mask.flipConnected "Voltear conectado"
tool.mask.blur "Difuminado"
tool.mask.sharpen "Nitidez"
tool.mask.thickness "Espesor"
tool.mask.polish "Suavidad de bordes"
tool.mask.engraveEmboss "Grabar / Relieve"
tool.mask.extract "Extraer"
tool.mask.split "Dividir"
tool.mask.closeMask "Acción de cierre (enmascarado):"
tool.mask.closeUnmask "Acción de cierre (desenmascarad):"
tool.mask.closeAction "Acción de cierre:"
tool.mask.closeActionNone "Ninguno"
tool.mask.closeActionFill "Rellenar"
tool.mask.closeActionShell "Caparazón"
tool.mask.closeActionLayer "Capa"
tool.mask.closeAction.help "-- Ninguno
Simplemente extrae la pieza y deja que la parte extraída se abra.

-- Rellenar
El agujero se rellena y se alisa.
No utilices esta opción para superficies planas.

-- Caparazón
Cierra la forma extraída utilizando el valor de espesor.

-- Capa
Extrae la diferencia de capa (solo submenú de capa)."

// ----------------------------------------------
// matrix (transform / gizmo)
tool.matrix "Matriz"
tool.matrix.clone "Clonar"
tool.matrix.action "Acción"
tool.matrix.action.help "-- Mover origen
Mueve la malla al origen mundial.

-- Restablecer
Restablece la transformación de malla a identidad.

-- Hornear
Aplica la matriz al vértice y restablece la matriz. Visualmente, nada debería cambiar."
tool.matrix.transformOperation "Operación de transformación"
tool.matrix.translation "Traducción"
tool.matrix.rotation "Rotación"
tool.matrix.scale "Escala"
tool.matrix.uniformScale "Escala uniforme"
tool.matrix.uniformScale.help "Nomad no puede admitir una escala no uniforme como transformación de objeto, por lo que se aplicará como transformación de vértice."
tool.matrix.moveToOrigin "Mover origen"
tool.matrix.resetTransform "Restablecer"
tool.matrix.bakeTransform "Hornear"
tool.matrix.applyMethod "Método:"
tool.matrix.applyMethodAuto "Auto"
tool.matrix.applyMethodVertex "Vértice"
tool.matrix.applyMethodObject "Objeto"
tool.matrix.applyMethod.help "-- Auto
Deja que Nomad elija entre el modo Vértice u Objeto.
Normalmente, se prefiere el modo objeto a menos que se habilite la simetría o si hay enmascaramiento en la malla.

-- Vértice
Los vértices se transforman individualmente.
Se tienen en cuenta la simetría y la máscara.
Para las primitivas que no están validadas, el modo de objeto es forzado.

-- Objeto
El objeto se transforma como un todo.
La simetría y la máscara se ignoran.
Si utiliza una escala no uniforme, se forzará el modo de vértice."

// ----------------------------------------------
// transform
tool.transform.multiTouch "Multitáctil"
tool.transform.multiTouch.help "Si esta opción está deshabilitada, solo puedes usar un modo (traducir, rotar, escalar) a la vez."

// ----------------------------------------------
// gizmo
tool.gizmo.size "Tamaño del widget"
tool.gizmo.linearRollThreshold "Umbral de balanceo tangente"
tool.gizmo.linearRollThreshold.help "Umbral de ángulo para elegir entre el método de rodillo lineal o circular.

El valor por encima de este umbral utilizará el rodillo circular.

Si prefiere el rodillo lineal (dirección de la tangente), simplemente establezca este valor en 90°."
tool.gizmo.autoHide "Ocultar en interacción"
tool.gizmo.tap "Mover pivote personalizado con una sola pulsación"
tool.gizmo.tap.help "Esta opción solo es efectiva en el modo de pivote personalizado (desactivado automáticamente).

-- Ninguno
No pasa nada al pulsar la malla.

-- Primer golpe
Mueve el aparato en la primera intersección.

-- Punzada media
Mueve el aparato en el promedio de las dos primeras intersecciones."
tool.gizmo.tapNone "Ninguno"
tool.gizmo.tapFirstHit "Primer golpe"
tool.gizmo.tapMiddleStab "Punzada media"

// ----------------------------------------------
// lathe
tool.lathe.axis "Eje"
tool.lathe.axis.fixed "Fijado"
tool.lathe.axis.dynamic "Dinámico"

// ----------------------------------------------
// tube
tool.tube.snap "Captura"
tool.tube.snap.all "Cada punto"
tool.tube.snap.startEnd "Iniciar y terminar"

// ----------------------------------------------
// trim
tool.hole "Relleno de agujero"
tool.hole.fillHoles "Rellenar agujeros"
// tool.hole.reproject "Reproyectar agujeros rellenos"
// tool.hole.reproject.help "Intenta volver a proyectar el agujero rellenado para que sigas más de cerca el corte.
// Sin embargo, solo funcionará para una proyección bastante simple."
tool.hole.bridges "Espacio de pantalla booleano"
tool.hole.bridges.help "Si esta opción está habilitada, puedes hacer agujeros en el volumen.
La pendiente de corte también seguirá más de cerca la forma de corte."
tool.hole.threshold "Epsilon de umbral"
tool.hole.threshold.help "Ajustar este valor podría ayudar con el algoritmo de rellenado de orificios."
tool.hole.smoothing "Suavidad del agujero"

// ----------------------------------------------
// smudge
tool.smudge.quality "Calidad"
tool.smudge.quality.help "Cambia la resolución de los píxeles proyectados, los valores más bajos significan trazos más rápidos."

// ----------------------------------------------
// trim / split / project / selMask
tool.shape "Forma"
tool.shape.rectangleSquare "Cuadrado"
tool.shape.rectangleCentered "Centrado"
tool.shape.ellipseCircle "Circulo"
tool.shape.ellipseCentered "Centrado"
tool.shape.lineRotateStep "Girar paso"

// ----------------------------------------------
// measure
tool.measure.goldenRatio "Mostrar radio dorado"

// ----------------------------------------------
// topology
topology "Topología"
// multires
topology.multires.title "Multiresolución"
topology.multires.title.help "Mantén la resolución múltiple de una malla.

Si realizas cambios en una resolución más baja, los detalles de las resoluciones más altas se reproyectarán cuando vuelva a cambiar.

Las capas están disponibles en todas las resoluciones."
topology.multiresReverse "Reverso"
topology.multiresReverse.confirm "No se pudo crear la subdivisión base.

La topología actual probablemente no es el resultado de una subdivisión."
topology.multiresSubdivide "Subdividir"
topology.multiresSubdivideConfirm "La malla tendrá vértices de $0M, ¿estás seguro?"
topology.multiresDeleteLower "Eliminar inferior"
topology.multiresDeleteHigher "Eliminar superior"
topology.multiresKeepTriangles "Mantener triángulos"
topology.multiresLinear "Subdivisión lineal"
topology.multiresLinear.help "Simplemente subdivide la malla sin aplicar suavizado"
// voxel
topology.voxel.title "Refuerzo de malla de Voxel"
topology.voxel.title.help "Refuerzo de malla muestreando la malla en una cuadrícula.

Si el objeto no está cerrado (estanco), se aplicará primero un algoritmo de rellenado de orificios.

Las capas se vuelven a inyectar después de volver a mallar, pero la calidad se degradará."
topology.voxelResolution "Resolucion"
topología.voxelRemesh "Reforza de malla"
topology.voxelSharp "Mantener bordes nítidos"
topology.voxelSharp.help "Esta opción es útil principalmente para operaciones booleanas primitivas simples.

Introducirá distorsión en alguna área debido a que los puntos se rompen en los bordes."
topology.voxelSubLevel "Construir multiresolución"
topology.voxelSubLevel.help "Puedes reconstruir una jerarquía de varias resoluciones a partir de la salida del remezclador de Voxeles.

También se ejecutará más rápido y usará menos memoria, especialmente si el valor de detalle de Voxel es alto.
Sin embargo, si el valor de detalle de Voxel es bajo y estás solicitando muchos niveles múltiples, perderás detalles."
// dynamic topology
topology.surfaceUniform "Reforzar malla"
topology.surfaceDetail "Detalle"
topology.surfaceDetail.help "A diferencia del refuerzo de malla del Voxel, el de superficie no requiere que la malla esté cerrada.

También puede admitir enmascaramiento para que pueda proteger una parte de la malla de los cambios de topología.

Las capas se actualizan correctamente."
topology.surfaceMethod "Método"
toplogy.surfaceMethodUniformisation "Uniformidad"
toplogy.surfaceMethodSubdivision "Subdivisión"
toplogy.surfaceMethodDecimation "Diezmado"
topology.surfaceMethod.help "Comportamiento de la topología dinámica:
- Uniformidad: añadir y eliminar detalles
- Subdivisión: añadir detalle
- Diezmado: eliminar detalles."
topology.surfaceUseMasking "Proteger área enmascarada"
topology.surfaceUseMasking.help "Las áreas enmascaradas protegerán la topología para que no se modifique."
topology.surfaceExtrapolate "Extrapolación de vértices"
// dynamic
topology.dynamic "Topología dinámica"
topology.dynamicActivate "Habilitado"
topology.dynamicActivate.help "Con la topología dinámica, las herramientas de escultura pueden subdividir o simplificar la malla localmente en tiempo real.

Esta característica puede tener un impacto notable en el rendimiento.

Las capas se actualizan correctamente."
topology.dynamicDetailLevel "Detalle"
topology.dynamicDetailEdge "Detalle"
topology.dynamicDetailMethod "Detalle basado en..."
topology.dynamicDetailMethodZoom "Zoom"
topology.dynamicDetailMethodRadius "Radio"
topology.dynamicDetailMethodConstant "Constante"
topology.dynamicDetailMethod.help "-- Zoom
El nivel de detalle se basa en lo lejos que se encuentre de la superficie.

-- Radio
El radio de la herramienta define la cantidad de detalle.

-- Constante
El detalle está fijado, el valor de detalle también se comparte con el control deslizante de Voxel."
topology.dynamicQuality "Prefer..."
topology.dynamicQuality.help "Si eliges la calidad, las dos diferencias principales son:
- el refinamiento se aplica antes que el operador de escultura, obtendrá menos artefactos de interpolación al pintar o esculpir detalles muy pequeños
- el refinamiento no se aplica de forma incremental, si esculpe detalles muy pequeños o realiza trazos rápidos, la topología siempre se refinará correctamente

Para un mejor rendimiento, y si planea usar esta opción, puedes considerar habilitar la opción \"dibujo parcial\" en el panel de configuración."
topology.dynamicQualitySpeed "Velocidad"
topology.dynamicQualityQuality "Calidad"
topology.dynamicUsePressure "Aplicar presión sobre el radio"
topology.dynamicUsePressure.help "Utiliza esta opción si deseas que el impacto de la presión del lápiz en el radio de la herramienta afecte al nivel de detalle."
// topology.dynamicBrush "Brocha"
// topology.dynamicGlobal "Global"
// topology.dynamicSettings "Configuración - Brocha / Global"
// decimate
topología.diezmar.titular "Diezmado"
topology.decimate.title.help "Reduce el número de polígonos intentando mantener la mayor cantidad de detalles posible..

Esta función puede ser útil si quieres exportar para una impresión 3D.
Sin embargo, probablemente no deberías usarlo si quieres seguir esculpiendo en él, ya que puede producir triángulos desiguales.

Ten en cuenta que el área enmascarada no se diezmará."
topology.decimate "Diezmar"
topology.decimateTargetFaces "Triángulos objetivos"
topology.decimatePaintWeight "Preservar pintura"
topology.decimatePaintWeight.help "Un valor más alto intentará preservar la pintura.

Establece este valor en 0 si no te importa la pintura."
topology.decimateUniform "Caras uniformes"
topology.decimateUniform.help "Un valor más alto generará triángulos de tamaño similar."
// topology.decimatePreserveBorders "Preservar bordes"
// topology.decimatePreserveBorders.help "No diezmes el borde de la malla.

// This is only relevant for object that are not watertight."

// BFF is activated through Debug menu
topology.uv.title "Desenvoltura automática UV"
topology.uvAtlas "Desenvolver Atlas"
topology.uvAtlas.warning "Puede ser muy lento, objetivo a < 100k vértices"
topology.uvBFF "Desenvolver Bff"
topology.uvBFF.warning "¡Puede tener superposiciones si la malla tiene asas!"
topology.uvBFFCones "Recuento de conos"
topology.uvBFFCones.help "Un valor más alto reducirá la distorsión para objetos complejos.

Un valor más alto significará un tiempo de cálculo más largo."
topology.uvDelete "Eliminar UV"

// baking
topology.bake "Hornear pintura de vértices"
topology.bake.help "Transfiere la pintura de vértices a una textura.

El color de los vértices se restablece en el proceso."
topology.bakeResolution "Resolucion"

// ----------------------------------------------
// privacy policy
privacyPolicy.title "Política de privacidad"
privacyPolicy.reject "Rechazar"
privacyPolicy "Hexanomad no recopila ningún dato de Nomad Sculpt."

// ----------------------------------------------
// version trial
version.buyWeb "La versión web es solo una demostración"
version.buyFull "Actualiza a la versión completa"
version.trialLimit "Versión de prueba:
- Deshacer/rehacer 3 veces
- Una capa por malla
- solo un proyecto activo
- sin importación / exportación"
version.restorePurchase "Restaurar compra"
version.fullFeatures "- Compra única
- Deshacer/rehacer ilimitado
- Capas ilimitadas
- Guardar y cargar
- Exportación e importación"