const { useCases } = require('../lib/vercel-use-cases.cjs')

module.exports = function handler(_request, response) {
  response.status(200).json(useCases)
}
