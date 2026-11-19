{{/*
Return whether HPA is enabled.
*/}}
{{- define "app.hpa.enabled" -}}
{{- if eq (tpl (toString .Values.hpa.enabled) $) "true" -}}
{{- true -}}
{{- end -}}
{{- end -}}

{{/*
Return the service account name.
*/}}
{{- define "app.serviceAccountName" -}}
{{- if .Values.serviceAccount.create -}}
{{- default (include "common.names.fullname" .) .Values.serviceAccount.name -}}
{{- else -}}
{{- default "default" .Values.serviceAccount.name -}}
{{- end -}}
{{- end -}}
