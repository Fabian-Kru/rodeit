<script setup lang="ts">
import { ApiReference } from '@scalar/api-reference';

import { specs } from '../config.js';
import { merge_specs } from '../util/openapi.ts';

const spec = merge_specs(specs);
</script>

<template>
	<ApiReference :configuration="{
		spec: { content: spec },
		isEditable: false,
	}" />
</template>
