-- Script de inicialização do banco de dados para Docker
-- Este arquivo é executado automaticamente quando o container PostgreSQL é criado

-- Criar usuário padrão se não existir
INSERT INTO users (id, name, email, password_hash, age) VALUES 
('550e8400-e29b-41d4-a716-446655440000', 'Usuário Padrão', 'admin@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/5Qz8K2', 30)
ON CONFLICT (id) DO NOTHING;

-- Inserir categorias padrão se não existirem
INSERT INTO categories (name, description) VALUES 
('Ficção', 'Livros de ficção e literatura'),
('Não-Ficção', 'Livros informativos e educativos'),
('Tecnologia', 'Livros sobre programação e tecnologia'),
('História', 'Livros históricos e biografias'),
('Ciência', 'Livros científicos e acadêmicos'),
('Autoajuda', 'Livros de desenvolvimento pessoal'),
('Romance', 'Livros românticos'),
('Mistério', 'Livros de suspense e mistério'),
('Fantasia', 'Livros de fantasia e ficção científica'),
('Poesia', 'Coleções de poesia')
ON CONFLICT (name) DO NOTHING;

-- Inserir alguns livros de exemplo
INSERT INTO books (title, author, isbn, description, content, category_id, user_id, is_public) VALUES 
(
    'O Guia do Mochileiro das Galáxias',
    'Douglas Adams',
    '978-0-345-39180-3',
    'Uma comédia de ficção científica sobre as aventuras de Arthur Dent no espaço.',
    'Capítulo 1: A Casa

Arthur Dent estava deitado na grama em frente à sua casa, olhando para o céu e pensando em como sua vida estava prestes a mudar para sempre. Ele não sabia que em poucos minutos sua casa seria demolida para dar lugar a uma rodovia interestelar.

Capítulo 2: A Demolição

Quando os tratores chegaram, Arthur tentou impedir a demolição, mas foi informado que a ordem vinha diretamente do Conselho Galáctico. Foi então que seu amigo Ford Prefect revelou que era na verdade um alienígena e que a Terra estava prestes a ser destruída.',
    (SELECT id FROM categories WHERE name = 'Ficção' LIMIT 1),
    '550e8400-e29b-41d4-a716-446655440000',
    true
),
(
    '1984',
    'George Orwell',
    '978-0-452-28423-4',
    'Um romance distópico sobre totalitarismo e controle social.',
    'Capítulo 1: O Grande Irmão

Era um dia frio e brilhante de abril, e os relógios batiam treze. Winston Smith, com o queixo colado ao peito para escapar do vento cortante, deslizou rapidamente pelas portas de vidro do Victory Mansions, mas não com rapidez suficiente para impedir que uma rajada de areia e poeira entrasse junto com ele.

O corredor cheirava a repolho cozido e tapetes velhos. Na parede do fundo, um cartaz colorido, muito grande demais para uso interno, estava afixado na parede. Representava simplesmente um rosto enorme, de mais de um metro de largura: o rosto de um homem de uns 45 anos, com bigode grosso e feições bonitas e brutais.',
    (SELECT id FROM categories WHERE name = 'Ficção' LIMIT 1),
    '550e8400-e29b-41d4-a716-446655440000',
    true
),
(
    'A Arte da Guerra',
    'Sun Tzu',
    '978-0-553-21305-0',
    'Um tratado militar chinês sobre estratégia e tática.',
    'Capítulo 1: Planejamento

A guerra é de vital importância para o Estado. É o reino da vida ou da morte, o caminho para a sobrevivência ou a ruína. É imperativo estudá-la profundamente.

Capítulo 2: Waging War

Quando você se envolve em uma guerra real, se a vitória demora muito para chegar, as armas ficam cegas e o moral afunda. Quando as tropas atacam cidades, sua força se esgota.',
    (SELECT id FROM categories WHERE name = 'História' LIMIT 1),
    '550e8400-e29b-41d4-a716-446655440000',
    true
)
ON CONFLICT DO NOTHING;
