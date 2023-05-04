{{/*
Generates configuration for a microservice from a default
*/}}
{{- define "saas-lib.microservice-config" -}}
{{- $defaults := deepCopy .context.Values.microservices._default -}}
{{- $overrides := index .context.Values.microservices .name -}}
{{- mergeOverwrite
        $defaults
        $overrides
        (dict
            "name" .name
            "labels" .labels
            "selectorLabels" .selectorLabels
        )
    | toYaml
-}}
{{- end }}
