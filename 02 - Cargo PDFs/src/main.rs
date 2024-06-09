fn main() {
    use pdf_composer::{FontsStandard, PaperOrientation, PaperSize, PDFComposer, PDFDocInfoEntry, PDFVersion};
    use std::path::PathBuf;
    
    // Create a new PDFComposer instance
    let mut my_pdf_instance = PDFComposer::new();
    
    // Add some paths. Relative paths
    let paths = vec![
        PathBuf::from("source_mds/file_01.md"),
        PathBuf::from("source_mds/file_02.md")
    ];
    my_pdf_instance.add_source_files(paths);
    
    // PDF version (not the version of the document, but the Adobe (formerly) PDF format version)
    my_pdf_instance.set_pdf_version(PDFVersion::V2_0);
    
    // Output directory for the generated PDFs
    my_pdf_instance.set_output_directory("example_pdfs");
    
    // Set the paper size
    my_pdf_instance.set_paper_size(PaperSize::A5);
    
    // Set the paper orientation
    my_pdf_instance.set_orientation(PaperOrientation::Landscape);
    
    // Set the page margins
    my_pdf_instance.set_margins("20");
    
    // Set font
    my_pdf_instance.set_font(FontsStandard::TimesRoman);
    
    // Metadata for the PDFs
    let author_entry = PDFDocInfoEntry {
        doc_info_entry: "Author",
        yaml_entry: "author",
    };
    let keywords_entry = PDFDocInfoEntry {
        doc_info_entry: "Keywords",
        yaml_entry: "keywords",
    };
    let subject_entry = PDFDocInfoEntry {
        doc_info_entry: "Subject",
        yaml_entry: "description",
    };
    let language_entry = PDFDocInfoEntry {
        doc_info_entry: "Language",
        yaml_entry: "language",
    };
    my_pdf_instance.set_doc_info_entry(author_entry);
    my_pdf_instance.set_doc_info_entry(keywords_entry);
    my_pdf_instance.set_doc_info_entry(subject_entry);
    my_pdf_instance.set_doc_info_entry(language_entry);
    
    // Generate the PDF(s)
    my_pdf_instance.generate_pdfs();
}
