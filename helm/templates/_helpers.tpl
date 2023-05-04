{{/*
Expand the name of the chart.
*/}}
{{- define "saas.name" -}}
{{- printf "%s-%s" (default .Chart.Name .Values.nameOverride) (default "" .name) | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "saas.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "saas.labels" -}}
helm.sh/chart: {{ include "saas.chart" . }}
{{ include "saas.selectorLabels" . }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "saas.selectorLabels" -}}
app.kubernetes.io/name: {{ include "saas.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}
