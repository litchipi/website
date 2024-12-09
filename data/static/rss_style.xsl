<?xml version="1.0" encoding="utf-8"?>
<!-- Taken from https://yusuf.fyi/styles.xsl -->
<xsl:stylesheet version="3.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
                xmlns:atom="http://www.w3.org/2005/Atom" xmlns:dc="http://purl.org/dc/elements/1.1/"
                xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
  <xsl:output method="html" version="1.0" encoding="UTF-8" indent="yes"/>
  <xsl:template match="/">
    <html lang="en">

      <head>
        <title><xsl:value-of select="/rss/channel/title"/> RSS Feed</title>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1"/>
        <link rel="stylesheet" type="text/css" href="/static/rss.css"/>
      </head>

      <body>
        <header class="banner">
          <img src="/static/logo.png"/>
          <h1><a href="/"><xsl:value-of select="/rss/channel/title"/></a></h1>
        </header>

        <p class="rss-intro">
          Enter this URL to the RSS feed reader of your choice:
          <code><xsl:value-of select="/rss/channel/link"/>/rss</code>
        </p>

        <ul class="rss-post-list">
          <xsl:for-each select="/rss/channel/item">
          <a target="_blank">
            <xsl:attribute name="href">
              <xsl:value-of select="link"/>
            </xsl:attribute>
            <li class="rss-item">
              <h3 class="rss-item-title">
                  <xsl:value-of select="title"/>
              </h3>
              <small class="rss-item-metadata">
                Published: <xsl:value-of select="pubDate" />
              </small>
              <p class="rss-item-description">
                <xsl:value-of select="description"/>
              </p>
            </li>
          </a>
          </xsl:for-each>
        </ul>
      </body>
    </html>
  </xsl:template>
</xsl:stylesheet>
