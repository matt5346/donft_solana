module.exports = {
  "root": true,
  "env": {
    "browser": true,
    "es2020": true,
    "node": true
  },
  "extends": [
    "plugin:vue/vue3-essential",
    "eslint:recommended"
  ],
  "parserOptions": {
    "parser": "@babel/eslint-parser"
  },
  "rules": {
    "indent": [
      2,
      2
    ],
    "semi": [
      2,
      "always"
    ],
    "quotes": [
      2,
      "double",
      {
        "avoidEscape": true
      }
    ],
    "vue/html-indent": [
      "error",
      2,
      {
        "alignAttributesVertically": true
      }
    ]
  }
};
