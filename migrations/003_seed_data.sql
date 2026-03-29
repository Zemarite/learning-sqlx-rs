-- Seed realistic data for Nexus Dynamics
-- 1. Create the Organization
INSERT INTO
	ORGANIZATIONS (ID, NAME, DESCRIPTION)
VALUES
	(
		GEN_RANDOM_UUID(),
		'Nexus Dynamics',
		'AI-powered collaboration and enterprise productivity platform'
	)
RETURNING
	ID;

-- Store the org_id for reference (we'll use a variable in a real script, or repeat the query)
DO $$
DECLARE
    org_id UUID;
    eng_id UUID;
    prod_id UUID;
    sales_id UUID;
    ops_id UUID;
    member_ids UUID[];
BEGIN
    SELECT id INTO org_id FROM organizations WHERE name = 'Nexus Dynamics';


-- 2. Create Divisions (one by one to capture IDs safely)
    INSERT INTO divisions (id, organization_id, name, description)
    VALUES (gen_random_uuid(), org_id, 'Engineering', 
            'Core software development, infrastructure, and AI research')
    RETURNING id INTO eng_id;

    INSERT INTO divisions (id, organization_id, name, description)
    VALUES (gen_random_uuid(), org_id, 'Product & Design', 
            'Product strategy, UX/UI, and customer experience')
    RETURNING id INTO prod_id;

    INSERT INTO divisions (id, organization_id, name, description)
    VALUES (gen_random_uuid(), org_id, 'Sales & Marketing', 
            'Go-to-market, customer acquisition, and brand')
    RETURNING id INTO sales_id;

    INSERT INTO divisions (id, organization_id, name, description)
    VALUES (gen_random_uuid(), org_id, 'Operations & HR', 
            'People, finance, legal, and internal operations')
    RETURNING id INTO ops_id;

	
-- 3. Create Members (15 realistic entries)
    INSERT INTO members (id, organization_id, division_id, name, email, role)
    VALUES 
        -- Engineering
  		(gen_random_uuid(), org_id, eng_id, 'Alex Rivera', 'alex.rivera@nexus-dynamics.com', 'Senior Backend Engineer'),
        (gen_random_uuid(), org_id, eng_id, 'Jordan Kim', 'jordan.kim@nexus-dynamics.com', 'AI Research Scientist'),
        (gen_random_uuid(), org_id, eng_id, 'Sam Patel', 'sam.patel@nexus-dynamics.com', 'DevOps Engineer'),
        (gen_random_uuid(), org_id, eng_id, 'Taylor Chen', 'taylor.chen@nexus-dynamics.com', 'Frontend Engineer'),
        
        
        -- Product & Design
        (gen_random_uuid(), org_id, prod_id, 'Morgan Ellis', 'morgan.ellis@nexus-dynamics.com', 'Product Manager'),
        (gen_random_uuid(), org_id, prod_id, 'Casey Quinn', 'casey.quinn@nexus-dynamics.com', 'Senior UX Designer'),
        (gen_random_uuid(), org_id, prod_id, 'Riley Brooks', 'riley.brooks@nexus-dynamics.com', 'Product Designer'),
        
        -- Sales & Marketing
        (gen_random_uuid(), org_id, sales_id, 'Jamie Torres', 'jamie.torres@nexus-dynamics.com', 'Head of Sales'),
        (gen_random_uuid(), org_id, sales_id, 'Avery Singh', 'avery.singh@nexus-dynamics.com', 'Marketing Specialist'),
        (gen_random_uuid(), org_id, sales_id, 'Parker Nguyen', 'parker.nguyen@nexus-dynamics.com', 'Account Executive'),
        
        -- Operations & HR
        (gen_random_uuid(), org_id, ops_id, 'Cameron Lee', 'cameron.lee@nexus-dynamics.com', 'HR Director'),
        (gen_random_uuid(), org_id, ops_id, 'Drew Harper', 'drew.harper@nexus-dynamics.com', 'Operations Manager'),
		-- example of a Select retreiving ID
        (gen_random_uuid(), org_id, (SELECT id FROM divisions WHERE name = 'Operations & HR'), 'Skylar Patel', 'skylar.patel@nexus-dynamics.com', 'Finance Analyst'),
        
        -- Organization-level / floating
        (gen_random_uuid(), org_id, NULL, 'Leslie Kim', 'leslie.kim@nexus-dynamics.com', 'CEO'),
        (gen_random_uuid(), org_id, NULL, 'Jordan Vale', 'jordan.vale@nexus-dynamics.com', 'CTO');

 -- 4. Create Groups
    -- Engineering groups
    INSERT INTO groups (id, organization_id, division_id, name, description)
    VALUES 
        (gen_random_uuid(), org_id, eng_id, 'Backend Core', 'Responsible for APIs, databases, and business logic'),
        (gen_random_uuid(), org_id, eng_id, 'AI & ML Guild', 'Research and integration of AI features'),
        (gen_random_uuid(), org_id, eng_id, 'Platform & Infra', 'Kubernetes, CI/CD, and cloud infrastructure');

    -- Product & Design groups
    INSERT INTO groups (id, organization_id, division_id, name, description)
    VALUES 
        (gen_random_uuid(), org_id, prod_id, 'Product Strategy', 'Roadmap planning and prioritization'),
        (gen_random_uuid(), org_id, prod_id, 'Design System', 'Component library and UX standards');

    -- Sales & Marketing groups
    INSERT INTO groups (id, organization_id, division_id, name, description)
    VALUES 
        (gen_random_uuid(), org_id, sales_id, 'Enterprise Sales', 'Large account deals and negotiations'),
        (gen_random_uuid(), org_id, sales_id, 'Content & Growth', 'Blog, social, and demand generation');

    -- Operations & HR groups
    INSERT INTO groups (id, organization_id, division_id, name, description)
    VALUES 
        (gen_random_uuid(), org_id, ops_id, 'People Ops', 'Recruiting, onboarding, and employee experience');

    -- Cross-organization / division groups
    INSERT INTO groups (id, organization_id, division_id, name, description)
    VALUES 
        (gen_random_uuid(), org_id, NULL, 'Innovation Council', 'Cross-functional ideas and R&D initiatives'),
        (gen_random_uuid(), org_id, NULL, 'Culture & Wellness', 'Company events, DEI, and well-being programs');

 -- 5. Assign Members to Groups (many-to-many)
    INSERT INTO member_groups (member_id, group_id)
    SELECT 
        m.id,
        g.id
    FROM members m
    CROSS JOIN groups g
    WHERE 
        -- Engineering members to Engineering groups
        (m.name IN ('Alex Rivera', 'Jordan Kim', 'Sam Patel', 'Taylor Chen') AND g.name IN ('Backend Core', 'AI & ML Guild', 'Platform & Infra'))
        OR
        -- Some overlap / multiple groups
        (m.name = 'Jordan Kim' AND g.name = 'Innovation Council')  -- AI researcher on council
        OR
        -- Product members
        (m.name IN ('Morgan Ellis', 'Casey Quinn', 'Riley Brooks') AND g.name IN ('Product Strategy', 'Design System'))
        OR
        -- Sales members
        (m.name IN ('Jamie Torres', 'Avery Singh', 'Parker Nguyen') AND g.name IN ('Enterprise Sales', 'Content & Growth'))
        OR
        -- Ops members
        (m.name IN ('Cameron Lee', 'Drew Harper', 'Skylar Patel') AND g.name = 'People Ops')
        OR
        -- Leadership on cross groups
        (m.name IN ('Leslie Kim', 'Jordan Vale') AND g.name IN ('Innovation Council', 'Culture & Wellness'))
        OR
        -- Additional realistic assignments (multiple groups per person)
        (m.name = 'Alex Rivera' AND g.name = 'Platform & Infra')
        OR
        (m.name = 'Casey Quinn' AND g.name = 'Innovation Council');

END $$;