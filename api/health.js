module.exports = function handler(_request, response) {
  response.status(200).json({
    status: 'ok',
    version: '0.1.0',
    uptime: Math.floor(process.uptime()),
  })
}
