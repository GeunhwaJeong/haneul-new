// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const defaultColors = require('tailwindcss/colors');
const defaultTheme = require('tailwindcss/defaultTheme');

module.exports = {
    content: ['./src/**/*.{js,jsx,ts,tsx}'],
    theme: {
        fontFamily: {
            sans: ['Inter', ...defaultTheme.fontFamily.sans],
            advanced: ['Inter', 'cursive'],
            mono: ['Space Mono', ...defaultTheme.fontFamily.mono],
        },
        colors: {
            haneul: '#6fbcf0',
            haneuldark: '#1670b8',
            offwhite: '#fefefe',
            haneullight: '#e6effe',
            haneuldarktwo: '#34526e',
            offblack: '#111111',
            ...defaultColors,
        },
    },
    plugins: [],
};
