import React, { useContext } from 'react';
import Article from '../components/general/ArticleCard';
import { RssItemsContext, ErrorsContext } from '../components/contexts/FeedProvider';

export default function About() {
  const { rssItems } = useContext(RssItemsContext); // Use context directly, no need to destructure
  const { errors } = useContext(ErrorsContext); // Use context directly, no need to destructure
  
  return (
    <div className="articles-container">
      <h1>Temp render of new feed items</h1>
      <hr />
      {rssItems.map((article, index) => (
        <Article key={index} title={article.title} description={article.description} author={"LV"} datetime={""} />
      ))}
      {errors.length > 0 && (
        <div>
          <h2>Errors:</h2>
          <ul>
            {errors.map((error, index) => (
              <li key={index}>{error}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
