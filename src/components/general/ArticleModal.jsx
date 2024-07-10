import React from 'react';
import parse, { domToReact } from 'html-react-parser';

export default function ArticleModal({ article, onClose }) {

  console.log(article);
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

  let content = article.RSS ? article.RSS.description : article.ATOM.summary;
  content = content ? parseWithClassNames(String(content)) : null;
  let link = article.RSS && article.RSS.link ? article.RSS.link : (article.ATOM && article.ATOM.link ? article.ATOM.link : 'No link available');

  return (
    <div className="modal">
      <div className="modal-content">
        <span className="close" onClick={onClose}>&times;</span>
        <h1>{article.RSS ? article.RSS.title : article.ATOM.title}</h1>
        <hr / >
        <a href="{link}"><small>{link} - This needs to be a tauri api call</small></a>
        <hr />
        <p>{content ? content : 'No summary available'}</p>
        <hr />
        <p>Entry by <b>{article.RSS ? article.RSS.author : article.ATOM.author}</b></p> 
        <p>Written on {article.RSS ? article.RSS.pub_date : article.ATOM.pub_date}</p>
        <p>From {article.RSS ? article.RSS.source : article.ATOM.id}</p>
        <p>Contributors [{article.RSS ? article.RSS.contributor : article.ATOM.contributor}]</p>
        <p>Hash: {article.RSS ? article.RSS.hash : article.ATOM.hash}</p>
        <div>
          <h4>Rights</h4>
          <span>{article.RSS ? article.RSS.rights : article.ATOM.rights}</span>
          <h4>Category: {article.RSS ? article.RSS.category : article.ATOM.category}</h4>
        </div>
      </div>
    </div>
  );
}
