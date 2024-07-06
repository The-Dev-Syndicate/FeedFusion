// src/pages/ArticleModal.js
import React from 'react';
import parse, { domToReact } from 'html-react-parser';

export default function ArticleModal({ article, onClose }) {
  const parseWithClassNames = (htmlContent) => {
    return parse(htmlContent, {
      replace: (domNode) => {
        if (domNode.type === 'tag') {
          const className = `article-${domNode.name}`;
          if (domNode.name === 'img') {
            return (
              <img
                {...domNode.attribs}
                className={className}
              />
            );
          } else {
            return React.createElement(
              domNode.name,
              { ...domNode.attribs, className },
              domNode.children ? domToReact(domNode.children, parseWithClassNames) : null
            );
          }
        }
      },
    });
  };

  let content = article.Rss ? article.Rss.description : article.Atom.summary;
  content = content ? parseWithClassNames(String(content)) : null;

  return (
    <div className="modal">
      <div className="modal-content">
        <span className="close" onClick={onClose}>&times;</span>
        <h1><small>{article.Rss ? article.Rss.category : article.Atom.category}</small> - {article.Rss ? article.Rss.title : article.Atom.title}</h1>
        <p><small>{article.Rss && article.Rss.link ? article.Rss.link : (article.Atom && article.Atom.link ? article.Atom.link : 'No link available')}</small></p>
        <p>{content ? content : 'No summary available'}</p>
        <p>Entry by <b>{article.Rss ? article.Rss.author : article.Atom.author}</b> on {article.Rss ? article.Rss.pub_date : article.Atom.pub_date}</p>
        <p>From {article.Rss ? article.Rss.source : article.Atom.id}</p>
        <p>Contributors [{article.Rss ? article.Rss.contributor : article.Atom.contributor}]</p>
        <p>Hash: {article.Rss ? article.Rss.hash : article.Atom.hash}</p>
        <div>
          <h4>Rights</h4>
          <span>{article.Rss ? article.Rss.rights : article.Atom.rights}</span>
        </div>
      </div>
    </div>
  );
}
