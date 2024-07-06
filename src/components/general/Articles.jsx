import React, { useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import ArticleCard from '../general/ArticleCard';
import { RssItemsContext, ErrorsContext } from '../contexts/FeedProvider';
import { SelectedFeedContext } from '../contexts/SelectedFeedContext';

export default function Articles() {
  const navigate = useNavigate();
  const { rssItems } = useContext(RssItemsContext);
  const { errors } = useContext(ErrorsContext);
  const { selectedFeed } = useContext(SelectedFeedContext);

  const handleCardClick = (title) => {
    console.log('Clicked index:', title);
    navigate(`/article/${title}`);
  };

  // Filter items based on selectedFeed
  const filteredItems = rssItems.filter((article) => {

  const rssLink = article.Rss && article.Rss.link;
  const atomLink = article.Atom && article.Atom.link;

  console.log('Selected Feed:', selectedFeed);
  console.log('RSS Link:', rssLink);
  console.log('Atom Link:', atomLink);
    if (!selectedFeed) {
      return true; // No filter, show all items
    }
    // Check if either Rss or Atom link starts with selectedFeed
    return (
      (article.Rss && article.Rss.link && article.Rss.link.startsWith(selectedFeed)) ||
      (article.Atom && article.Atom.link && article.Atom.link.startsWith(selectedFeed))
    );
  });

  return (
    <div className="articles-container">
      {filteredItems.length > 0 ? (
        filteredItems.map((article, index) => (
          <div key={index} onClick={() => handleCardClick(article.Rss ? article.Rss.title : article.Atom.title)}>
            <ArticleCard
              title={article.Rss ? article.Rss.title : article.Atom.title}
              date={article.Rss ? article.Rss.pub_date : article.Atom.pub_date}
              author={article.Rss ? article.Rss.author : article.Atom.author}
              description={article.Rss ? article.Rss.description : article.Atom.summary}
            />
          </div>
        ))
      ) : (
        <p>No articles available for the selected feed.</p>
      )}
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
